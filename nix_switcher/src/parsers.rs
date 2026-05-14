use crate::generator::*;
use std::fs;
use std::io;
use walkdir::WalkDir;
use serde::Deserialize;
use std::path::*; 

pub fn pars_config() -> Data {
    let json_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("config.json").display().to_string();
    let file_content = fs::read_to_string(&json_path).expect("Datei konnte nicht gelesen werden");
    let loaded_config: Data = serde_json::from_str(&file_content).unwrap();
    loaded_config
}

pub fn pars_themecolor(file: String) -> Colors {
    let content: String = fs::read_to_string(file).expect("Konnte die Theme Datei nicht lesen");
    let colors: Colors = serde_json::from_str(&content).expect("JSON-Format der Datei ist ungültig");
    colors
}

pub fn pars_themes() -> Vec<String> {
    let folder_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("themes").to_str().unwrap().to_string();
    WalkDir::new(&folder_path)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter_map(|entry| {
            entry.path().file_name().and_then(|n| n.to_str()).map(|s| s.to_string())
        })
    .collect()
}

pub fn pars_themefiles() -> Vec<String> {
    let folder_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("themes").to_str().unwrap().to_string();
    WalkDir::new(&folder_path)
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn pars_kitty() -> Vec<String> {
    WalkDir::new(gen_path(PathType::Kittythemes))
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().file_stem().and_then(|n| n.to_str()).map(|s| s.to_string())
        })
        .collect()
}

pub fn pars_wallpaper() -> Vec<String> {
    WalkDir::new(&pars_config().wallpaperdir)
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn pars_wallfile() -> Vec<String> { 
    let file = PathBuf::from(gen_path(PathType::Nixswitcher)).join("wallpaper.json").display().to_string();
    let file_content = fs::read_to_string(&file).expect("Konnte Wallpaper.json nicht parsen");
    let loaded_content: Vec<String> = serde_json::from_str(&file_content).expect("Konnte Wallpaper.json nicht formatieren");
    loaded_content
}

pub fn pars_gwallpath(index: usize) -> String {
    let wallpaper = pars_wallfile();
    if index >=wallpaper.len() {
        panic!("So viele Wallpaper stehen nicht zur verfügung. Das Maximum sind: {}", wallpaper.len() - 1);
    }
    wallpaper[index].clone()
}

pub fn pars_links() -> Config {
    let file = PathBuf::from(gen_path(PathType::Nixswitcher)).join("links.json").display().to_string();
    let file_content = fs::read_to_string(&file)
        .expect("Wallpapers.json konnte nicht geparst werden");
    let loaded_config: Config = serde_json::from_str(&file_content)
        .expect("Konnte Wallpapers.json nicht konvertieren");
    loaded_config
}


pub fn pars_recent() -> Recent {
    let file = PathBuf::from(gen_path(PathType::Nixswitcher)).join("recent.json").display().to_string();
    let file_content = fs::read_to_string(&file)
        .expect("recent.json konnte nicht geparst werden");
    let loaded_config: Recent = serde_json::from_str(&file_content)
        .expect("Konnte recent.json nicht konvertieren");
    loaded_config
}

pub fn pars_rwallpath(index: usize, theme: &str) -> String {
    let config = pars_links();
    if let Some(wallpaper_info) = config.theme.get(theme) {
        if index < wallpaper_info.wallpapers.len() {
            return wallpaper_info.wallpapers[index].clone();
        } else {
            panic!("So viele Wallpaper sind für dieses Theme nicht verlinkt");
        } 
    } else {
        panic!("Dieses Theme wurde nicht gefunden");
    }
}


