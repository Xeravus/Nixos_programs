use log::{debug, info, error};
use std::process::{self, Command};
use std::fs;
use std::collections::HashMap;

pub struct Drives {
    pub medium: HashMap<String, Partition>,
}

pub struct Partition {
    pub partitions: String,
    pub size: String,
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

pub fn pars_drives() -> Drives {
    let mut drives = Drives {
        medium: HashMap::new(),
    };

    let fdisk = Command::new("sudo")
        .arg("fdisk")
        .arg("-l")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte fdisk nicht starten: {}", err); process::exit(1); });
    if !fdisk.status.success() {
        let err = String::from_utf8_lossy(&fdisk.stderr);
        error!("[ FAILED ] - Fehler beim Auslesen der Partitionen: {}", err);
        process::exit(1);
    }

    let output = String::from_utf8_lossy(&fdisk.stdout);
    debug!("fdisk output: \n{}", output);
    for i in output.lines() {
        if !i.starts_with("Disk /dev/") {
            continue;
        }
        debug!("gefundende Zeilen: {}", i);
        if let Some((links, rechts)) = i.split_once(", ") {
            let name = links.replace("Disk /dev/", "");
            if let Some((groesse, _rest)) = rechts.split_once(", ") {
                let partition = Partition {
                    partitions: String::from("TBD"),
                    size: groesse.to_string(),
                };
                drives.medium.insert(name, partition);
            }
        }
    }
    drives
}

pub fn tailscale_fetch() -> Taildevices {

    let tail_status = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte 'tailscale status --json' nicht ausführen: {}", err); process::exit(1); });
    if !tail_status.status.success() {
        let err = String::from_utf8_lossy(&tail_status.stderr);
        error!("[ FAILED ] - Tailscale Status ist Fehlgeschlagen, bist du eingelogt, wurde das JSON nicht richtig geparst: {}", err);
        process::exit(1);
    }

    info!("[ OK ] - Fetched Tailscale Devices");
    serde_json::from_slice::<Taildevices>(&tail_status.stdout)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte den Output von Tailscale nicht parsen: {}", err); process::exit(1); })
}

