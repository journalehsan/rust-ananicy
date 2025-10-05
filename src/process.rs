use std::fs;
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use procfs::process::{Process, Stat};
use nix::unistd::Pid;
use nix::sys::signal;
use log::{debug, warn};
use crate::rules::Rule;
use crate::cgroup::CgroupController;

pub struct ProcessInfo {
    process: Process,
    stat: Stat,
    cmdline: Vec<String>,
}

impl ProcessInfo {
    pub fn new(pid: i32) -> Result<Self> {
        let process = Process::new(pid)?;
        let stat = process.stat()?;
        let cmdline = process.cmdline()
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();
        
        Ok(Self { process, stat, cmdline })
    }
    
    pub fn pid(&self) -> i32 {
        self.stat.pid
    }
    
    pub fn name(&self) -> &str {
        &self.stat.comm
    }
    
    pub fn nice(&self) -> i32 {
        self.stat.nice as i32
    }
    
    pub fn set_nice(&self, nice: i32) -> Result<()> {
        unsafe {
            let result = libc::setpriority(libc::PRIO_PROCESS, self.pid() as u32, nice);
            if result != 0 {
                return Err(anyhow!("Failed to set nice value"));
            }
        }
        debug!("Set nice {} for process {}", nice, self.pid());
        Ok(())
    }
    
    pub fn set_oom_score_adj(&self, score: i32) -> Result<()> {
        let path = format!("/proc/{}/oom_score_adj", self.pid());
        fs::write(&path, score.to_string())?;
        debug!("Set OOM score {} for process {}", score, self.pid());
        Ok(())
    }
    
    pub fn set_ionice(&self, class: Option<&str>, level: Option<i32>) -> Result<()> {
        let mut cmd = std::process::Command::new("ionice");
        cmd.arg("-p").arg(self.pid().to_string());
        
        if let Some(class) = class {
            let class_num = match class {
                "none" | "best-effort" => 2,
                "idle" => 3,
                "realtime" => 1,
                _ => 2,
            };
            cmd.arg("-c").arg(class_num.to_string());
        }
        
        if let Some(level) = level {
            cmd.arg("-n").arg(level.to_string());
        }
        
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(anyhow!("Failed to set ionice"));
        }
        
        debug!("Set ionice for process {}", self.pid());
        Ok(())
    }
    
    pub fn set_scheduler(&self, sched: &str, rtprio: Option<i32>) -> Result<()> {
        let mut cmd = std::process::Command::new("schedtool");
        
        let sched_arg = match sched {
            "other" | "normal" => "-N",
            "rr" => "-R",
            "fifo" => "-F",
            "batch" => "-B",
            "iso" => "-I",
            "idle" => "-D",
            _ => "-N",
        };
        
        cmd.arg(sched_arg);
        
        if let Some(prio) = rtprio {
            if sched == "rr" || sched == "fifo" {
                cmd.arg("-p").arg(prio.to_string());
            }
        }
        
        cmd.arg(self.pid().to_string());
        
        let output = cmd.output()?;
        if !output.status.success() {
            warn!("Failed to set scheduler for process {}", self.pid());
        } else {
            debug!("Set scheduler {} for process {}", sched, self.pid());
        }
        
        Ok(())
    }
    
    pub fn apply_rule(&self, rule: &Rule, cgroups: &HashMap<String, CgroupController>) -> Result<()> {
        if let Some(nice) = rule.nice {
            self.set_nice(nice)?;
        }
        
        if rule.ioclass.is_some() || rule.ionice.is_some() {
            self.set_ionice(rule.ioclass.as_deref(), rule.ionice)?;
        }
        
        if let Some(ref sched) = rule.sched {
            self.set_scheduler(sched, rule.rtprio)?;
        }
        
        if let Some(oom) = rule.oom_score_adj {
            self.set_oom_score_adj(oom)?;
        }
        
        if let Some(ref cgroup_name) = rule.cgroup {
            if let Some(cgroup) = cgroups.get(cgroup_name) {
                cgroup.add_pid(self.pid())?;
            }
        }
        
        Ok(())
    }
    
    pub fn matches_rule(&self, rule: &Rule) -> bool {
        // Check by process name
        if let Some(ref rule_name) = rule.name {
            if self.name() != rule_name && self.process.exe().ok()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str()) != Some(rule_name) {
                return false;
            }
        }
        
        // Check by cmdline patterns
        if let Some(ref cmdlines) = rule.cmdlines {
            for pattern in cmdlines {
                if !self.cmdline.iter().any(|arg| arg.contains(pattern)) {
                    return false;
                }
            }
        }
        
        true
    }
}

pub fn scan_processes() -> Result<Vec<ProcessInfo>> {
    let mut processes = Vec::new();
    
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        let name = entry.file_name();
        
        if let Some(name_str) = name.to_str() {
            if let Ok(pid) = name_str.parse::<i32>() {
                if let Ok(proc_info) = ProcessInfo::new(pid) {
                    processes.push(proc_info);
                }
            }
        }
    }
    
    Ok(processes)
}

pub fn scan_and_apply_rules(
    rules: &[Rule], 
    cgroups: &HashMap<String, CgroupController>
) -> Result<usize> {
    let processes = scan_processes()?;
    let mut applied = 0;
    
    for proc in processes {
        for rule in rules {
            if proc.matches_rule(rule) {
                if let Err(e) = proc.apply_rule(rule, cgroups) {
                    warn!("Failed to apply rule to PID {}: {}", proc.pid(), e);
                } else {
                    applied += 1;
                }
                break; // Apply only first matching rule
            }
        }
    }
    
    Ok(applied)
}