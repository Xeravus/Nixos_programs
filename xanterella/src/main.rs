mod ssh;

use ssh::*;

use std::process::Command;
use std::path;
use std::collections::HashMap;
use notify_rust::Notification;
use clap::{Parser, Subcommand};
use log::{debug, info, error};
use env_logger::init;
use serde_json::*;
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

#[derive(serde::Deserialize, Debug)]
pub struct Taildevices {
    #[serde(rename = "Peer")]
    pub devices: HashMap<String, DeviceInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DeviceInfo {
    #[serde(rename = "HostName")]
    pub name: String,
    #[serde(rename = "TailscaleIPs")]
    pub ip: Vec<String>,
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
    ssh_ping(&select_host(tailscale_fetch()));
}

pub fn tailscale_fetch() -> Taildevices {
    let tail_status = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .expect("Konnte den Befehl: tailscale status --json nicht ausführen");
    if tail_status.status.success() {
        info!("[ OK ] - Fetched Tailscale Devices");
    } else if !tail_status.status.success() {
        error!("[ FAILED ] - Tailscale Status ist Fehlgeschlagen, bist du eingelogt, wurde das JSON nicht richtig geparst, ...");
        panic!("Abbruch");
    }
    serde_json::from_slice::<Taildevices>(&tail_status.stdout)
        .expect("Konnte das Tailscale-JSON nicht parsen")
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
            panic!("Abbruch");
        }
    }
    output_ip 
}
