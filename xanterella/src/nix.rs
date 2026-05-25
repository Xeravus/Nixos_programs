use log::{info, error};
use std::process::{self, Command};

use crate::generator::*;

pub fn nix_install(ip: &String) {
    let host = format!("root@{}", ip);
    Command::new("nix")
        .arg("run")
        .arg("github:numtide/nixos-anywhere")
        .arg("--")
        .arg("--flake")
        .arg(".#crylia")
        .arg(host)
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte nixos-anywhere nicht ausführen: {}", err); process::exit(1); });
    info!("[ OK ] - Hat Nixos auf Zielgerät installiert");
}
