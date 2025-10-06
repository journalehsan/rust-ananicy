use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_check_freq")]
    pub check_freq: f64,
    
    #[serde(default = "default_true")]
    pub verbose: bool,
    
    #[serde(default)]
    pub verbose_opts: VerboseOpts,
    
    #[serde(default = "default_config_dir")]
    pub config_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerboseOpts {
    #[serde(default = "default_true")]
    pub cgroup_load: bool,
    
    #[serde(default = "default_true")]
    pub type_load: bool,
    
    #[serde(default = "default_true")]
    pub rule_load: bool,
    
    #[serde(default = "default_true")]
    pub apply_nice: bool,
    
    #[serde(default = "default_true")]
    pub apply_ioclass: bool,
    
    #[serde(default = "default_true")]
    pub apply_ionice: bool,
    
    #[serde(default = "default_true")]
    pub apply_sched: bool,
    
    #[serde(default = "default_true")]
    pub apply_oom_score_adj: bool,
    
    #[serde(default = "default_true")]
    pub apply_cgroup: bool,
}

fn default_check_freq() -> f64 { 5.0 }
fn default_true() -> bool { true }
fn default_config_dir() -> String { "/etc/ananicy.d/".to_string() }

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self> {
        let config_file = config_dir.join("ananicy.conf");
        
        if !config_file.exists() {
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(&config_file)?;
        let config = Self::parse_config(&content)?;
        
        Ok(config)
    }
    
    fn parse_config(content: &str) -> Result<Self> {
        let mut config = Self::default();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "check_freq" => {
                        config.check_freq = value.parse()?;
                    }
                    "verbose" => {
                        config.verbose = parse_bool(value);
                    }
                    "cgroup_load" => {
                        config.verbose_opts.cgroup_load = parse_bool(value);
                    }
                    "type_load" => {
                        config.verbose_opts.type_load = parse_bool(value);
                    }
                    "rule_load" => {
                        config.verbose_opts.rule_load = parse_bool(value);
                    }
                    "apply_nice" => {
                        config.verbose_opts.apply_nice = parse_bool(value);
                    }
                    "apply_ioclass" => {
                        config.verbose_opts.apply_ioclass = parse_bool(value);
                    }
                    "apply_ionice" => {
                        config.verbose_opts.apply_ionice = parse_bool(value);
                    }
                    "apply_sched" => {
                        config.verbose_opts.apply_sched = parse_bool(value);
                    }
                    "apply_oom_score_adj" => {
                        config.verbose_opts.apply_oom_score_adj = parse_bool(value);
                    }
                    "apply_cgroup" => {
                        config.verbose_opts.apply_cgroup = parse_bool(value);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_freq: default_check_freq(),
            verbose: true,
            verbose_opts: VerboseOpts::default(),
            config_dir: default_config_dir(),
        }
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "true" | "yes" | "1")
}