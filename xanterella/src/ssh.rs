use std::process::{self, Command};
use log::{debug, info, error};

pub fn ssh_ping(ip: &String) {
    let ping = Command::new("ping")
        .args(["-c", "1"])
        .args(["-W", "1"])
        .arg(ip)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Ping nicht starten: {}", err); process::exit(1); });

    if !ping.status.success() {
        error!("[ FAILED ] - Konnte das Gerät nicht pingen: {}", ip);
        panic!("Abbruch");
    }

    info!("[ OK ] - Ping erfolgreich");
    let ssh_command = format!("root@{}", ip);
    let ssh = Command::new("ssh")
        .arg(&ssh_command)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Tailscale nicht starten: {}", err); process::exit(1); });
    if !ssh.status.success() {
        error!("[ FAILED ] - Konnte das Gerät nicht über ssh erreichen: {}", ssh_command);
        panic!("Abbruch");
    }
    info!("[ OK ] - SSH-PING erfolgreich");
}

pub fn ssh_get_hardware(ip: &String) -> String {
    let ssh_command = format!("root@{}", ip);
    let ssh = Command::new("ssh")
        .arg(&ssh_command)
        .arg("nixos-generate-config --no-filesystems --show-hardware-config")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Hardware Config nicht erstellen: {}", err); process::exit(1); });

    if !ssh.status.success() {
        let err = String::from_utf8_lossy(&ssh.stderr);
        error!("[ FAILED ] - Fehler beim erstellen der Hardware Config: {}", err);
        panic!("Abbruch");
    }

    let hardware_config = String::from_utf8_lossy(&ssh.stdout).to_string();
    info!("[ OK ] - Hardware Config erstellt");
    debug!("{}", hardware_config);
    hardware_config
}
