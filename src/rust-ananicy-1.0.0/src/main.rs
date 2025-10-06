mod config;
mod process;
mod rules;
mod cgroup;
mod utils;

use std::time::Duration;
use std::path::Path;
use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{info, error};
use tokio::time;

#[derive(Parser)]
#[command(name = "rust-ananicy")]
#[command(about = "Auto Nice Daemon - Rust Implementation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration directory
    #[arg(short, long, default_value = "/etc/ananicy.d/")]
    config_dir: String,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the daemon
    Start,
    
    /// Dump information
    Dump {
        #[command(subcommand)]
        target: DumpTarget,
    },
}

#[derive(Subcommand)]
enum DumpTarget {
    /// Show loaded rules
    Rules,
    /// Show loaded types
    Types,
    /// Show cgroups
    Cgroups,
    /// Show processes
    Proc,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    let config = config::Config::load(Path::new(&cli.config_dir))?;
    
    match cli.command {
        Commands::Start => {
            info!("Starting Rust Ananicy daemon");
            run_daemon(config).await?;
        }
        Commands::Dump { target } => {
            match target {
                DumpTarget::Rules => {
                    let rules = rules::load_rules(Path::new(&cli.config_dir))?;
                    println!("{}", serde_json::to_string_pretty(&rules)?);
                }
                DumpTarget::Types => {
                    let types = rules::load_types(Path::new(&cli.config_dir))?;
                    println!("{}", serde_json::to_string_pretty(&types)?);
                }
                DumpTarget::Cgroups => {
                    let cgroups = cgroup::load_cgroups(Path::new(&cli.config_dir))?;
                    for (name, cg) in cgroups {
                        println!("Cgroup: {} - CPU Quota: {}%", name, cg.cpu_quota());
                    }
                }
                DumpTarget::Proc => {
                    let processes = process::scan_processes()?;
                    for proc in processes {
                        println!("PID: {} - Name: {} - Nice: {}", 
                                 proc.pid(), proc.name(), proc.nice());
                    }
                }
            }
        }
    }
    
    Ok(())
}

async fn run_daemon(config: config::Config) -> Result<()> {
    let mut interval = time::interval(Duration::from_secs(config.check_freq as u64));
    let rules = rules::load_all_rules(Path::new(&config.config_dir))?;
    let cgroups = cgroup::load_cgroups(Path::new(&config.config_dir))?;
    
    // Notify systemd we're ready
    if std::env::var("NOTIFY_SOCKET").is_ok() {
        std::process::Command::new("systemd-notify")
            .arg("--ready")
            .spawn()?;
    }
    
    loop {
        interval.tick().await;
        
        match process::scan_and_apply_rules(&rules, &cgroups) {
            Ok(applied) => {
                if config.verbose {
                    info!("Applied rules to {} processes", applied);
                }
            }
            Err(e) => error!("Error applying rules: {}", e),
        }
    }
}