use log::{info, error};
use std::process::{self, Command};

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

pub fn nix_install(ip: &String) {
    let host = format!("root@{}", ip);
    let folder_path = "/home/cato/nixos-config/";
    Command::new("nix")
        .arg("run")
        .arg("github:numtide/nixos-anywhere")
        .arg("--")
        .arg("--flake")
        .arg(".#crylia")
        .arg(host)
        .current_dir(&folder_path)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte nixos-anywhere nicht ausführen: {}", err); process::exit(1); });
    info!("[ OK ] - Hat Nixos auf Zielgerät installiert");
}
