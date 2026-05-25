mod check;
mod files;
//mod gen;
mod get;
mod git;
mod nix;
mod parse;

use check::*;
use files::*;
use get::*;
use git::*;
use nix::*;
use parse::*;

use std::process::{self, Command};
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use log::{debug, info, error};
use inquire::Select;

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Hostname,
    Ping {
        ip: String,
    },
    Clean,
    Debug {
        #[arg(short, long)]
        tailfetch: bool,
        #[arg(short, long)]
        selecthost: bool,
        #[arg(short, long)]
        gethardware: bool,
    },
    RemoteInstall,
}

pub fn main() {
    let cli = Cli::parse();
    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .init();
    match &cli.command {
        Commands::Hostname => {
            let _ = Command::new("hostname") .spawn();
        },
        Commands::Ping { ip } => {
            ssh_ping(ip);
        },
        Commands::Clean => {
            files_crylia_finish();
            git_full(String::from("Xanterella Remote-Install cleanup"));
        },
        Commands::Debug { tailfetch, selecthost, gethardware } => {
            if *tailfetch {
                debug!("{:?}", tailscale_fetch());
            } else if *selecthost {
                debug!("{}", select_host(tailscale_fetch()));
            } else if *gethardware {
                ssh_get_hardware(&String::from("127.0.0.1"));
            } else {
                debug!("No Args")
            }
        },
        Commands::RemoteInstall => {
            remote_install();
        },
    }
}

pub fn remote_install() {
    let target_ip = select_host(tailscale_fetch());
    ssh_ping(&target_ip);
    ssh_get_hardware(&target_ip);
    files_crylia_start(ssh_get_hardware(&target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    nix_check();
    nix_install(&target_ip);
    // -----------------------------------------------------
    files_crylia_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}

pub fn select_host(hosts: Taildevices) -> String {
    let mut options: Vec<String> = vec![];
    let mut output_ip: String = String::from("127.0.0.1");
    for (_pubkey, device_info) in hosts.devices {
        let ip: &str = device_info.ip.first().map(|s| s.as_str()).unwrap_or("Keine IP");
        let input = format!("IP: {:<15} - Name: {}", ip, device_info.name);
        options.push(input);
    }
    let answer = Select::new("Select Hosts", options).prompt();
    match answer {
        Ok(choice) => {
            if let Some((ip, _name)) = choice.split_once(" - Name: ") {
                let clean_ip: &str = ip.strip_prefix("IP: ").unwrap_or(ip).trim();
                output_ip = String::from(clean_ip);
            }
        },
        Err(e) => {
            error!("[ FAILED ] - Konnte den Input nicht auslesen: {}", e);
            process::exit(1);
        }
    }
    debug!("Output IP: {}", output_ip);
    output_ip 
}
