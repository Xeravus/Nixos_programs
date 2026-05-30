use log::{debug, info, error};
use std::process::{self, Command};
use std::fs;

use crate::generator::*;

pub fn files_crylia_start(config: String) {
    let file_path1 = "/home/cato/nixos-config/hosts/crylia/configuration.nix";
    let file_path2 = "/home/cato/nixos-config/hosts/crylia/hardware-configuration.nix";
    fs::write(&file_path2, &config)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Hardware Config nicht schreiben: {}", err); process::exit(1); });
    info!("[ OK ] - Hardware Config für Crylia erstellt");
    let content = fs::read_to_string(file_path1)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); process::exit(1); });
    let Some((anfang, ende)) = content.split_once("  imports = [") else {
        error!("[ FAILED ] - Konnte 'imports = [' nicht in der Config von Crylia finden");
        process::exit(1);
    };
    let whole_content = format!(
        "{}
        imports = [
        ./hardware-configuration.nix
        {}", anfang, ende);

    debug!("Neuer Inhalt: \n{}", whole_content);
    fs::write(&file_path1, &whole_content)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); process::exit(1); });
    info!("[ OK ] - Configuration von Crylia überschreiben");
    files_alejandra();
}

pub fn files_crylia_finish() {
    let file_path1 = "/home/cato/nixos-config/hosts/crylia/configuration.nix";
    let file_path2 = "/home/cato/nixos-config/hosts/crylia/hardware-configuration.nix";
    let content = fs::read_to_string(file_path1)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); process::exit(1); });
    let whole_content = content.replace("    ./hardware-configuration.nix\n", "");
    debug!("Neuer Inhalt: \n{}", whole_content);
    fs::write(&file_path1, &whole_content)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); process::exit(1); });
    info!("[ OK ] - Configuration von Crylia überschreiben");
    fs::remove_file(file_path2)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Hardware Config nicht löschen: {}", err); process::exit(1); });
    info!("[ OK ] - Hardware Config gelöscht");
    files_alejandra();
}

pub fn files_alejandra() {
    let alejandra = Command::new("alejandra")
        .arg(".")
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err); process::exit(1); });
    debug!("Alejandra: \n{}", String::from_utf8_lossy(&alejandra.stdout));
    if !alejandra.status.success() {
        let err = String::from_utf8_lossy(&alejandra.stderr);
        error!("[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}", err);
        process::exit(1);
    }
    info!("[ OK ] - Dateien wurden mit Alejandra formatiert");
}
