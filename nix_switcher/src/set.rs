#[warn(unused_imports)]
use crate::generator::*;
use crate::parsers::*;
use crate::notify_user;
use serde::*;
use std::fs;
use std::path::*;

pub fn set_global(theme: &str, index: usize) -> Data {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(theme),
        wallpaper: pars_gwallpath(index),
        ..structin
    };
    structout
}

pub fn set_relativ(intheme: &str) -> Data {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(intheme),
        wallpaper: pars_rwallpath(pars_recentwall(intheme), intheme),
        ..structin
    };
    structout
}

pub fn set_theme(theme: &str) -> Data {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(theme),
        ..structin
    };
    let string = format!("{} -> {}", &structin.theme, &theme);
    notify_user(&string, &string);
    structout
}

pub fn set_wall(index: usize) -> Data {
    let structin = pars_config();
    let structout = Data {
        wallpaper: pars_rwallpath(index, &structin.theme),
        ..structin
    };
    set_recent(index, &structout.theme);
    structout
}

pub fn set_kittytheme(theme: String) -> Data {
    let structin = pars_config();
    let structout = Data {
        kittytheme: String::from(theme),
        ..structin
    };
    structout
}

pub fn set_recent(wallpaperindex: usize, theme: &str) {
    let mut recent = pars_recent();
    if let Some(recent_info) = recent.theme.get_mut(theme) {
        recent_info.wallpaper = wallpaperindex;
    } else {
        println!("Fehler: Das Theme '{}' existiert in den recents nicht.", theme);
    }
    let file_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("recent.json").to_str().unwrap().to_string();
    let json_string = serde_json::to_string_pretty(&recent).unwrap();
    fs::write(&file_path, &json_string).expect("Konnte wallpapers.json nicht überschreiben");
}

pub fn change(structin: Data) {
    let json_string = serde_json::to_string_pretty(&structin).unwrap();
    let json_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("config.json").to_str().unwrap().to_string();
    fs::write(&json_path, &json_string).expect("Konnte config.json nicht beschreiben");
}
