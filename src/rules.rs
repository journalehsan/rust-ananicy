use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: Option<String>,
    
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    
    pub nice: Option<i32>,
    
    pub ioclass: Option<String>,
    
    pub ionice: Option<i32>,
    
    pub sched: Option<String>,
    
    pub rtprio: Option<i32>,
    
    pub oom_score_adj: Option<i32>,
    
    pub cgroup: Option<String>,
    
    pub cmdlines: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pub name: String,
    
    pub nice: Option<i32>,
    
    pub ioclass: Option<String>,
    
    pub ionice: Option<i32>,
    
    pub sched: Option<String>,
    
    pub rtprio: Option<i32>,
    
    pub oom_score_adj: Option<i32>,
    
    pub cgroup: Option<String>,
}

pub fn load_types(config_dir: &Path) -> Result<HashMap<String, Type>> {
    let mut types = HashMap::new();
    
    for entry in walkdir(config_dir)? {
        if let Some(file_name) = entry.file_name() {
            if file_name.to_string_lossy().ends_with(".types") {
                info!("Loading types from: {:?}", entry);
                let content = fs::read_to_string(&entry)?;
            
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                if let Ok(type_def) = serde_json::from_str::<Type>(line) {
                    types.insert(type_def.name.clone(), type_def);
                }
            }
        }
    }
    }
    
    Ok(types)
}

pub fn load_rules(config_dir: &Path) -> Result<Vec<Rule>> {
    let mut rules = Vec::new();
    
    for entry in walkdir(config_dir)? {
        if let Some(file_name) = entry.file_name() {
            if file_name.to_string_lossy().ends_with(".rules") {
                info!("Loading rules from: {:?}", entry);
                let content = fs::read_to_string(&entry)?;
            
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                if let Ok(rule) = serde_json::from_str::<Rule>(line) {
                    rules.push(rule);
                }
            }
        }
    }
    }
    
    Ok(rules)
}

pub fn load_all_rules(config_dir: &Path) -> Result<Vec<Rule>> {
    let types = load_types(config_dir)?;
    let mut rules = load_rules(config_dir)?;
    
    // Apply type definitions to rules
    for rule in &mut rules {
        if let Some(ref type_name) = rule.type_name {
            if let Some(type_def) = types.get(type_name) {
                // Merge type properties into rule (rule properties take precedence)
                if rule.nice.is_none() {
                    rule.nice = type_def.nice;
                }
                if rule.ioclass.is_none() {
                    rule.ioclass = type_def.ioclass.clone();
                }
                if rule.ionice.is_none() {
                    rule.ionice = type_def.ionice;
                }
                if rule.sched.is_none() {
                    rule.sched = type_def.sched.clone();
                }
                if rule.rtprio.is_none() {
                    rule.rtprio = type_def.rtprio;
                }
                if rule.oom_score_adj.is_none() {
                    rule.oom_score_adj = type_def.oom_score_adj;
                }
                if rule.cgroup.is_none() {
                    rule.cgroup = type_def.cgroup.clone();
                }
            }
        }
    }
    
    Ok(rules)
}

fn walkdir(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
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