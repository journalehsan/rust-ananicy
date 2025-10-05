use std::fs;
use std::path::Path;
use anyhow::Result;
use log::warn;

pub fn check_disk_schedulers() {
    let block_dir = Path::new("/sys/class/block");
    
    if let Ok(entries) = fs::read_dir(block_dir) {
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            // Skip loop, ram, and sr devices
            if name_str.contains("loop") || 
               name_str.contains("ram") || 
               name_str.contains("sr") {
                continue;
            }
            
            let scheduler_path = entry.path().join("queue/scheduler");
            if scheduler_path.exists() {
                if let Ok(content) = fs::read_to_string(&scheduler_path) {
                    if !content.contains("[cfq]") && 
                       !content.contains("[bfq]") && 
                       !content.contains("[bfq-mq]") {
                        warn!(
                            "Disk {} not using cfq/bfq scheduler - IO priorities may not work",
                            name_str
                        );
                    }
                }
            }
        }
    }
}

pub fn validate_nice(nice: i32) -> Result<i32> {
    if nice < -20 || nice > 19 {
        anyhow::bail!("Nice value must be between -20 and 19");
    }
    Ok(nice)
}

pub fn validate_ionice(ionice: i32) -> Result<i32> {
    if ionice < 0 || ionice > 7 {
        anyhow::bail!("IOnice value must be between 0 and 7");
    }
    Ok(ionice)
}

pub fn validate_rtprio(rtprio: i32) -> Result<i32> {
    if rtprio < 1 || rtprio > 99 {
        anyhow::bail!("RT priority must be between 1 and 99");
    }
    Ok(rtprio)
}

pub fn validate_oom_score_adj(adj: i32) -> Result<i32> {
    if adj < -1000 || adj > 1000 {
        anyhow::bail!("OOM score adjustment must be between -1000 and 1000");
    }
    Ok(adj)
}