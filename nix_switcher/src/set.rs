#[warn(unused_imports)]
use crate::generator::*;
use crate::parsers::*;
use serde::*;
use std::fs;

pub fn set_global(theme: &str, index: usize) -> Data {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(theme),
        wallpaper: pars_gwallpath(index),
        ..structin
    };
    structout
}

pub fn set_relativ(theme: &str, index: usize) -> Data {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(theme),
        wallpaper: pars_rwallpath(index, theme.to_string()),
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
    structout
}

pub fn set_wall(index: usize) -> Data {
    let structin = pars_config();
    let structout = Data {
        wallpaper: pars_rwallpath(index, structin.theme.clone()),
        ..structin
    };
    structout
}

pub fn change(structin: Data) {
    let json_string = serde_json::to_string_pretty(&structin).unwrap();
    let json_path: String = format!("{}/config.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte config.json nicht beschreiben");
}
