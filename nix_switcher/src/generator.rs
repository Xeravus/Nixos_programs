use crate::parsers::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::env; use std::path::Path; use indexmap::IndexMap;
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub theme: String,
    pub kittytheme: String,
    pub wallpaper: String,
    pub themedir: String,
    pub wallpaperdir: String,
    pub templatedir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: IndexMap<String, Wallpapers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpapers {
    pub wallpapers: Vec<String>,
}

#[derive(Deserialize)]
pub struct Colors {
    pub bg0: String,
    pub bg1: String,
    pub bg2: String,
    pub fg0: String,
    pub ac0: String,
    pub ac1: String,
    pub ac2: String,
}

pub fn gen_path(option: u8) -> String {
    let home = env::var("HOME").expect("Konnte die Homevariable nicht finden");
    let option1: String = format!("{}/.config/rice", home);
    let option2: String = format!("{}/nix-switcher", option1);
    let option3: String = format!("{}/kittythemes", option2);
    let option4: String = format!("{}/themes", option2);
    match option {
        1 => home,
        2 => option1,
        3 => option2,
        4 => option3,
        5 => option4,
        _ => panic!("Keine Option, für diesen Pfad"),
    }
}

pub fn gen_file_init() {
    gen_file_config();
    gen_file_links();
    gen_file_wallpaper();
    gen_kitty_themes();
}

pub fn gen_file_config() {
    let themedir = format!("{}/themes", gen_path(3));
    let wallpaperdir = format!("{}/wallpaper", gen_path(3));
    let templatedir = format!("{}/template", gen_path(3));
    let basic_conf = Data {
        theme: String::from("dracula"),
        kittytheme: String::from("dracula"),
        wallpaper: String::from("dracula"),
        themedir: themedir,
        wallpaperdir: wallpaperdir,
        templatedir: templatedir,
    };
    let json_string = serde_json::to_string_pretty(&basic_conf).unwrap();
    let json_path: String = format!("{}/config.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}

pub fn gen_file_links() {
    let mut theme_map: IndexMap<String, Wallpapers> = IndexMap::new();
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

pub fn gen_kitty_themes() {
    let json_string = serde_json::to_string_pretty(&pars_kitty()).unwrap();
    let json_path: String = format!("{}/kittythemes.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}

pub fn gen_theme(hexcodes: &Colors, themename: &str) {
    let qs_in: String = format!("{}/quickshell_template.qml", pars_config().templatedir);
    let hl_in: String = format!("{}/hyprland_template.conf", pars_config().templatedir);
    let qs_out: String = format!("{}/{}/quickshell_template.qml", gen_path(5), themename);
    let hl_out: String = format!("{}/{}/hyprland_template.conf", gen_path(5), themename);
    let in_paths: Vec<String> = vec![ qs_in, hl_in ];
    let out_paths: Vec<String> = vec![ qs_out, hl_out ];
    for (in_p, out_p) in in_paths.iter().zip(out_paths.iter()) {
        let in_content: String = fs::read_to_string(in_p).expect("Fehler beim Lesen der Templates");
        let out_content: String = in_content
            .replace("varbg0", &hexcodes.bg0)
            .replace("varbg1", &hexcodes.bg1)
            .replace("varbg2", &hexcodes.bg2)
            .replace("varfg0", &hexcodes.fg0)
            .replace("varac0", &hexcodes.ac0)
            .replace("varac1", &hexcodes.ac1)
            .replace("varac2", &hexcodes.ac2);
        fs::write(out_p, out_content).expect("Fehler beim schreiben der Templates");
        println!("Erfolgreich Template Gemacht: {}", in_p);
    }
}

pub fn gen_themes_all() {
    for i in pars_themefiles() {
        let stem = Path::new(&i)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .unwrap();
        gen_theme(&pars_themecolor(i), &stem);
    }
}
