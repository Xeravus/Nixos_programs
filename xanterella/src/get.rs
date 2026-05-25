use std::process::{self, Command};
use log::{debug, info, error};

use crate::generator::*;

pub fn ssh_get_hardware(ip: &String) -> String {
    let ssh_command = format!("root@{}", ip);
    let ssh = Command::new("ssh")
        .arg(&ssh_command)
        .arg("nixos-generate-config --no-filesystems --show-hardware-config")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte SSH nicht starten: {}", err); process::exit(1); });
    if !ssh.status.success() {
        let err = String::from_utf8_lossy(&ssh.stderr);
        error!("[ FAILED ] - Fehler beim erstellen der Hardware Config: {}", err);
        process::exit(1);
    }

    let hardware_config = String::from_utf8_lossy(&ssh.stdout).to_string();
    info!("[ OK ] - Hardware Config erstellt");
    debug!("{}", hardware_config);
    hardware_config
}
