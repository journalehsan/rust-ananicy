use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::info;

const CGROUP_FS: &str = "/sys/fs/cgroup";
const PERIOD_US: u64 = 100000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CgroupVersion {
    V1,
    V2,
}

#[derive(Debug, Clone)]
pub struct CgroupController {
    name: String,
    path: PathBuf,
    version: CgroupVersion,
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
        let version = detect_cgroup_version();
        let ncpu = num_cpus::get() as u64;
        let quota_us = PERIOD_US * ncpu * (cpu_quota as u64) / 100;
        let cpu_shares = 1024 * (cpu_quota as u64) / 100;

        match version {
            CgroupVersion::V1 => {
                let cpu_path = Path::new(CGROUP_FS).join("cpu").join(&name);
                if !cpu_path.exists() {
                    fs::create_dir_all(&cpu_path)?;
                }
                fs::write(cpu_path.join("cpu.cfs_period_us"), PERIOD_US.to_string())?;
                fs::write(cpu_path.join("cpu.cfs_quota_us"), quota_us.to_string())?;
                fs::write(cpu_path.join("cpu.shares"), cpu_shares.to_string())?;
                Ok(Self { name, path: cpu_path, version, cpu_quota, quota_us, cpu_shares })
            }
            CgroupVersion::V2 => {
                let base = v2_delegated_base()?;
                enable_cpu_controller_v2_at(&base)?;
                let v2_path = base.join(&name);
                if !v2_path.exists() {
                    fs::create_dir_all(&v2_path)?;
                }
                let max_value = if cpu_quota >= 100 { String::from("max") } else { quota_us.to_string() };
                let cpu_max = format!("{} {}", max_value, PERIOD_US);
                fs::write(v2_path.join("cpu.max"), cpu_max)?;
                let mut weight: u64 = (cpu_quota as u64) * 100;
                if weight == 0 { weight = 1; }
                if weight > 10000 { weight = 10000; }
                fs::write(v2_path.join("cpu.weight"), weight.to_string())?;
                Ok(Self { name, path: v2_path, version, cpu_quota, quota_us, cpu_shares })
            }
        }
    }
    
    pub fn add_pid(&self, pid: i32) -> Result<()> {
        match self.version {
            CgroupVersion::V1 => {
                let tasks_file = self.path.join("tasks");
                fs::write(&tasks_file, pid.to_string())?;
            }
            CgroupVersion::V2 => {
                let procs_file = self.path.join("cgroup.procs");
                fs::write(&procs_file, pid.to_string())?;
            }
        }
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

fn detect_cgroup_version() -> CgroupVersion {
    let v2_marker = Path::new(CGROUP_FS).join("cgroup.controllers");
    if v2_marker.exists() { CgroupVersion::V2 } else { CgroupVersion::V1 }
}

fn enable_cpu_controller_v2_at(base: &Path) -> Result<()> {
    let controllers_path = base.join("cgroup.controllers");
    if !controllers_path.exists() {
        return Ok(());
    }

    let available = fs::read_to_string(&controllers_path).unwrap_or_default();
    if !available.split_whitespace().any(|c| c == "cpu") {
        return Ok(());
    }

    let subtree = base.join("cgroup.subtree_control");
    let current = fs::read_to_string(&subtree).unwrap_or_default();
    let has_cpu = current.split_whitespace().any(|c| c == "+cpu" || c == "cpu");
    if !has_cpu {
        let _ = fs::write(&subtree, "+cpu");
    }
    Ok(())
}

fn v2_delegated_base() -> Result<PathBuf> {
    // Parse /proc/self/cgroup to find our cgroup v2 path and use it as base
    let content = fs::read_to_string("/proc/self/cgroup").unwrap_or_default();
    for line in content.lines() {
        // v2 lines look like: 0::/system.slice/rust-ananicy.service
        if let Some(rest) = line.split("::").nth(1) {
            let rel = rest.trim();
            if rel.starts_with('/') {
                return Ok(Path::new(CGROUP_FS).join(rel.trim_start_matches('/')));
            }
        }
    }
    // Fallback to root if parsing failed
    Ok(PathBuf::from(CGROUP_FS))
}