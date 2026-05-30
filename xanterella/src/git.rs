use log::{debug, info, error};
use std::process::{self, Command};

use crate::generator::*;

pub enum Branches {
    Xanterella,
    Main,
}

pub fn git_full(cm_msg: String) {

    let diff = Command::new("git")
        .args(["diff", "--stat"])
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
    if !diff.status.success() {
        let err = String::from_utf8_lossy(&diff.stderr);
        error!("[ FAILED ] - Git diff hat nicht funktioniert: {}", err);
        process::exit(1);
    }

    debug!("{}", String::from_utf8_lossy(&diff.stdout));

    let add = Command::new("git")
        .args(["add", "-A"])
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
    if !add.status.success() {
        let err = String::from_utf8_lossy(&add.stderr);
        error!("[ FAILED ] - Git add hat nicht funktioniert: {}", err);
        process::exit(1);
    }

    info!("[ OK ] - Dateien wurden Git hinzuigefügt");

    let commit = Command::new("git")
        .args(["commit", "-am", &cm_msg])
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
    if !commit.status.success() {
        let err = String::from_utf8_lossy(&commit.stderr);
        error!("[ FAILED ] - Git commit hat nicht funktioniert: {}", err);
        process::exit(1);
    }

    info!("[ OK ] - Änderungen Commited");
}

pub fn git_checkout(branch: Branches) {
    match branch {
        Branches::Xanterella => {
            let checkout = Command::new("git")
                .arg("checkout")
                .arg("xanterella")
                .current_dir(gen_path(Paths::Nixconf))
                .output()
                .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
            if !checkout.status.success() {
                error!("[ FAILED ] - Konnte die Branch nicht wechseln");
                info!("[ OK ] - Branch wird erstellt");

                let create = Command::new("git")
                    .args(["checkout", "-b", "xanterella"])
                    .current_dir(gen_path(Paths::Nixconf))
                    .output()
                    .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
                if !create.status.success() {
                    let err = String::from_utf8_lossy(&create.stderr);
                    error!("[ FAILED ] - Fehler beim erstellen der Branch: Xanterella: {}", err);
                    process::exit(1);
                }

                info!("[ OK ] - Branch xanterella erstellt und gewechselt");
            }
        },
        Branches::Main => {
            let checkout = Command::new("git")
                .arg("checkout")
                .arg("main")
                .current_dir(gen_path(Paths::Nixconf))
                .output()
                .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
            if !checkout.status.success() {
                error!("[ FAILED ] - Konnte die Branch nicht wechseln");
                info!("[ OK ] - Branch wird erstellt");

                let create = Command::new("git")
                    .args(["checkout", "-b", "main"])
                    .current_dir(gen_path(Paths::Nixconf))
                    .output()
                    .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
                if !create.status.success() {
                    let err = String::from_utf8_lossy(&create.stderr);
                    error!("[ FAILED ] - Fehler beim erstellen der Branch: Main: {}", err);
                    process::exit(1);
                }

                info!("[ OK ] - Branch main erstellt und gewechselt");
            }
        },
    }
}


pub fn git_auto_pr(added_host: String) {
    let title_message = format!("Xanterella: Added Host: {}", added_host);
    let body_message = format!("
        Xanterella hat einen neuen Host hinzugefügt
        Added Host: {}
        Diese Pull Request wurde durch 'git_auto_pr' getriggert
        ", added_host);
    git_checkout(Branches::Xanterella);

    let pr = Command::new("gh")
        .arg("pr")
        .arg("create")
        .args(["-B", "main"])
        .args(["-t", &title_message])
        .args(["-b", &body_message])
        .arg("--no-maintainer-edit")
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Git nicht starten: {}", err); process::exit(1); });
    if !pr.status.success() {
        let err = String::from_utf8_lossy(&pr.stderr);
        error!("[ FAILED ] - Fehler beim erstellen der PR: {}", err);
        process::exit(1);
    }

    git_checkout(Branches::Main);
}
