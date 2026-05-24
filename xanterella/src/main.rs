mod ssh;

use ssh::*;

use std::process::Command;
use std::path;
use std::collections::HashMap;
use notify_rust::Notification;
use clap::{Parser, Subcommand};
use serde_json::*;
use inquire::Select;

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    match &cli.command {
        Commands::Hostname => {
            let _ = Command::new("hostname") .spawn();
        },
        Commands::Ping { ip } => {
            ssh_ping(ip);
        },
        Commands::Debug { tailfetch, selecthost } => {
            if *tailfetch {
                println!("{:?}", tailscale_fetch());
            } else if *selecthost {
                select_host(tailscale_fetch());
            } else {
                println!("No Args")
            }
        },
        Commands::RemoteInstall => {
        },
    }
}
/*
pub fn remote_install() {
}
*/
pub fn tailscale_fetch() -> Taildevices {
    let tail_status = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .expect("Konnte den Befehl: tailscale status --json nicht ausführen");
    if !tail_status.status.success() {
        panic!("[ FAILED ] - Tailscale Status ist Fehlgeschlagen, bist du eingelogt, wurde das JSON nicht richtig geparst, ...");
    }
    serde_json::from_slice::<Taildevices>(&tail_status.stdout)
        .expect("Konnte das Tailscale-JSON nicht parsen")
}

pub fn select_host(hosts: Taildevices) {
    let mut options: Vec<String> = vec![];
    for (_pubkey, device_info) in hosts.devices {
        let ip: &str = device_info.ip.first().map(|s| s.as_str()).unwrap_or("Keine IP");
        let input = format!("IP: {} - Name: {}", ip, device_info.name);
        options.push(input);
    }
    let answer = Select::new("Select Hosts", options).prompt();
    match answer {
        Ok(choice) => {
            println!("Deine Auswahl: {}", choice);
            if let Some((ip, _name)) = choice.split_once(" - Name: ") {
                let clean_ip: &str = ip.strip_prefix("IP: ").unwrap_or(ip).trim();
                println!("IP: {}", clean_ip);
            }
        },
        Err(e) => {
            panic!("[ FAILED ] - Konnte den Input nicht auslesen: {}", e);
        }
    }
}
