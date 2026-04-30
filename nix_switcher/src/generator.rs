mod parsers;

use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub theme: String,
    pub themedir: String,
    pub wallpaper: String,
    pub wallpaperdir: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: HashMap<String, Wallpapers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpapers {
    pub wallpapers: Vec<String>,
}

pub fn gen_path(option: u8) -> String {
    let home = env::var("HOME").expect("Konnte die Homevariable nicht finden");
    let option1: String = format!("{}/.config/rice", home);
    let option2: String = format!("{}/nix-switcher", option1);
    match option {
        1 => home,
        2 => option1,
        3 => option2,
        _ => panic!("Keine Option, für diesen Pfad"),
    }
}

pub fn gen_file_init() {
    gen_file_config();
    gen_file_links();
    gen_file_wallpaper();
}

pub fn gen_file_config() {
    let themedir = format!("{}/themes", gen_path(2));
    let wallpaperdir = format!("{}/wallpaper", gen_path(2));
    let basic_conf = Data {
        theme: String::from("dracula"),
        themedir: themedir,
        wallpaper: String::from("dracula"),
        wallpaperdir: wallpaperdir,
    };
    let json_string = serde_json::to_string_pretty(&basic_conf).unwrap();
    let json_path: String = format!("{}/config.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}

pub fn gen_file_links() {
    let mut theme_map: HashMap<String, Wallpapers> = HashMap::new();
    for i in pars_themes() {
        theme_map.insert(
            i,
            Wallpapers {
                wallpapers: vec![],
            },
        );
    };
    let config_out = Config {
        theme: theme_map,
    };
    let json_string_wallpaper = serde_json::to_string_pretty(&config_out).unwrap();
    let json_path_wallpaper: String = format!("{}/links.json", gen_path(3));
    fs::write(&json_path_wallpaper, &json_string_wallpaper).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path_wallpaper);
}

pub fn gen_file_wallpaper() {
    let json_string = serde_json::to_string_pretty(&pars_wallpaper()).unwrap();
    let json_path: String = format!("{}/wallpaper.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}
