use crate::parsers::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::env; 
use std::path::*; 
use indexmap::IndexMap;

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

pub enum PathType {
    Riceconfig,
    Nixswitcher,
    Kittythemes,
    Themes,
    Config,
}

pub fn gen_path(option: PathType) -> String {
    let home = env::var("HOME").expect("Konnte die Homevariable nicht finden");
    let config = PathBuf::from(&home).join(".config");
    let riceconfig = PathBuf::from(&home).join(".config").join("rice");
    let nixswitcher = PathBuf::from(&riceconfig).join("nix-switcher");
    let kittythemes = PathBuf::from(&nixswitcher).join("kittythemes");
    let themes = PathBuf::from(&nixswitcher).join("themes");
    let result: PathBuf = match option {
        PathType::Riceconfig => riceconfig,
        PathType::Nixswitcher => nixswitcher,
        PathType::Kittythemes => kittythemes,
        PathType::Themes => themes,
        PathType::Config => config,
    };
    result.to_str().expect("Gen path hat fehlgeschlagen").to_string()
}

pub fn gen_file_init() {
    gen_file_config();
    gen_file_links();
    gen_file_wallpaper();
    gen_kitty_themes();
}

pub fn gen_file_config() {
    let basic_conf = Data {
        theme: String::from("dracula"),
        kittytheme: String::from("dracula"),
        wallpaper: String::from("dracula"),
        themedir: PathBuf::from(gen_path(PathType::Themes)).display().to_string(),
        wallpaperdir: PathBuf::from(gen_path(PathType::Riceconfig)).join("wallpaper").to_str().unwrap().to_string(),
        templatedir: PathBuf::from(gen_path(PathType::Nixswitcher)).join("template").to_str().unwrap().to_string(),
    };
    let json_string = serde_json::to_string_pretty(&basic_conf).unwrap();
    let json_path: String = PathBuf::from(gen_path(PathType::Nixswitcher)).join("config.json").display().to_string();
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
    let json_path_wallpaper: String = PathBuf::from(gen_path(PathType::Nixswitcher)).join("links.json").display().to_string();
    fs::write(&json_path_wallpaper, &json_string_wallpaper).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path_wallpaper);
}

pub fn update_file_links() {
    let link_file = PathBuf::from(gen_path(PathType::Nixswitcher)).join("links.json");
    let old_config: Option<Config> = if link_file.exists() {
        Some(pars_links())
    } else {
        None
    };
    let mut theme_map: IndexMap<String, Wallpapers> = IndexMap::new();
    for i in pars_themefiles() {
        let mut wallpapers_to_keep = vec![];
        if let Some(old) = &old_config {
            if let Some(old_theme_data) = old.theme.get(&i) {
                wallpapers_to_keep = old_theme_data.wallpapers.clone();
            }
        }
        theme_map.insert(
            i,
            Wallpapers {
                wallpapers: wallpapers_to_keep,
            },
        );
    }
    let config_out = Config {
        theme: theme_map,
    };
    let json_string_wallpaper = serde_json::to_string_pretty(&config_out).unwrap();
    fs::write(&link_file, &json_string_wallpaper).expect("Konnte Datei nicht schreiben");
}

pub fn gen_file_wallpaper() {
    let json_string = serde_json::to_string_pretty(&pars_wallpaper()).unwrap();
    let json_path: String = PathBuf::from(gen_path(PathType::Nixswitcher)).join("wallpaper.json").display().to_string();
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}

pub fn gen_kitty_themes() {
    let json_string = serde_json::to_string_pretty(&pars_kitty()).unwrap();
    let json_path: String = PathBuf::from(gen_path(PathType::Nixswitcher)).join("kittythemes.json").display().to_string();
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
}

pub fn gen_theme(hexcodes: &Colors, themename: &str) {
    let qs_in = PathBuf::from(pars_config().templatedir).join("quickshell_template.qml").to_str().unwrap().to_string();
    let hl_in = PathBuf::from(pars_config().templatedir).join("hyprland_template.conf").to_str().unwrap().to_string();
    let qs_out = PathBuf::from(gen_path(PathType::Themes)).join(themename).join("quickshell_template.qml").to_str().unwrap().to_string();
    let hl_out = PathBuf::from(gen_path(PathType::Themes)).join(themename).join("hyprland_template.conf").to_str().unwrap().to_string();
    let path = PathBuf::from(gen_path(PathType::Themes)).join(themename).display().to_string();
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
        let _ =fs::create_dir_all(&path);
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
