use log::{info, error};
use std::process::{self, Command};

pub fn git_full(cm_msg: String) {
    let folder_path = "/home/cato/nixos-config";
    Command::new("git")
        .args(["add", "-A"])
        .current_dir(folder_path)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Dateien git nicht hinzufügen: {}", err); process::exit(1); });
    info!("[ OK ] - Dateien wurden Git hinzuigefügt");
    Command::new("git")
        .args(["commit", "-am", &cm_msg])
        .current_dir(folder_path)
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte keinen Commit machen: {}", err); process::exit(1); });
    info!("[ OK ] - Änderungen Commited");
}

