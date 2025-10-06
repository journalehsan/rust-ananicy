use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::info;

const CGROUP_FS: &str = "/sys/fs/cgroup";
const PERIOD_US: u64 = 100000;

#[derive(Debug, Clone)]
pub struct CgroupController {
    name: String,
    path: PathBuf,
    cpu_quota: u32,
    quota_us: u64,
    cpu_shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CgroupDef {
    cgroup: String,
    #[serde(rename = "CPUQuota")]
    cpu_quota: u32,
}

impl CgroupController {
    pub fn new(name: String, cpu_quota: u32) -> Result<Self> {
        let cpu_path = Path::new(CGROUP_FS).join("cpu").join(&name);
        
        if !cpu_path.exists() {
            fs::create_dir_all(&cpu_path)?;
        }
        
        let ncpu = num_cpus::get() as u64;
        let quota_us = PERIOD_US * ncpu * (cpu_quota as u64) / 100;
        let cpu_shares = 1024 * (cpu_quota as u64) / 100;
        
        // Write cgroup settings
        fs::write(cpu_path.join("cpu.cfs_period_us"), PERIOD_US.to_string())?;
        fs::write(cpu_path.join("cpu.cfs_quota_us"), quota_us.to_string())?;
        fs::write(cpu_path.join("cpu.shares"), cpu_shares.to_string())?;
        
        Ok(Self {
            name,
            path: cpu_path,
            cpu_quota,
            quota_us,
            cpu_shares,
        })
    }
    
    pub fn add_pid(&self, pid: i32) -> Result<()> {
        let tasks_file = self.path.join("tasks");
        fs::write(&tasks_file, pid.to_string())?;
        Ok(())
    }
    
    pub fn cpu_quota(&self) -> u32 {
        self.cpu_quota
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn load_cgroups(config_dir: &Path) -> Result<HashMap<String, CgroupController>> {
    let mut cgroups = HashMap::new();
    
    for entry in walkdir(config_dir)? {
        if let Some(file_name) = entry.file_name() {
            if file_name.to_string_lossy().ends_with(".cgroups") {
                info!("Loading cgroups from: {:?}", entry);
                let content = fs::read_to_string(&entry)?;
            
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                if let Ok(cgroup_def) = serde_json::from_str::<CgroupDef>(line) {
                    let controller = CgroupController::new(
                        cgroup_def.cgroup.clone(),
                        cgroup_def.cpu_quota
                    )?;
                    cgroups.insert(cgroup_def.cgroup, controller);
                }
            }
        }
    }
    }
    
    Ok(cgroups)
}

fn walkdir(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                files.extend(walkdir(&path)?);
            } else {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}