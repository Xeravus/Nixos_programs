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

pub fn nix_check() {
    let folder_path = "/home/cato/nixos-config/";
    let check = Command::new("nixos-rebuild")
        .arg("dry-build")
        .arg("--flake")
        .arg(".#crylia")
        .current_dir(&folder_path)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Flake nicht checken: {}", err); process::exit(1); });
    if check.status.success() {
        info!("[ OK ] - Nix Flake ist funktionstüchtig");
    } else {
        let err = String::from_utf8_lossy(&check.stderr);
        error!("[ FAILED ] - Die Nix Flake ist nicht funktionierend: {}", err);
        process::exit(1);
    }
}

