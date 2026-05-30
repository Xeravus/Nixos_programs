use log::{debug, info, error};
use std::process::{self, Command};
use std::fs;
use std::env;
use std::path::*;

pub enum Paths {
    Nixconf,
}

pub fn gen_path(option: Paths) -> String {
    let home = env::var("HOME").expect("[ FAILED ] - Konnte die Home Variable nicht extrahieren");
    let nixconfig = PathBuf::from(&home).join("nixos-config");
    let result: PathBuf = match option {
        Paths::Nixconf => nixconfig,
    };
    result.to_str().expect("[ FAILED ] - Gen Path ist fehlgeschlagen").to_string()
}
