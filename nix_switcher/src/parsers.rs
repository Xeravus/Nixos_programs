use crate::generator::*;
use std::fs;
use walkdir::WalkDir;


pub fn pars_config() -> Data {
    let json_path: String = format!("{}/config.json", gen_path(3));
    let file_content = fs::read_to_string(&json_path).expect("Datei konnte nicht gelesen werden");
    let loaded_config: Data = serde_json::from_str(&file_content).unwrap();
    loaded_config
}

pub fn pars_wallpaper() -> Vec<String> {
    let folder_path: String = format!("{}/wallpaper", gen_path(2));
    WalkDir::new(&folder_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

pub fn pars_wallfile() -> Vec<String> {
    let file: String = format!("{}/wallpaper.json", gen_path(3));
    let file_content = fs::read_to_string(&file).expect("Konnte Wallpaper.json nicht parsen");
    let loaded_content: Vec<String> = serde_json::from_str(&file_content).expect("Konnte Wallpaper.json nicht formatieren");
    loaded_content
}

pub fn pars_gwallpath(index: usize) -> String {
    let wallpaper = pars_wallpaper();
    if index >=wallpaper.len() {
        panic!("So viele Wallpaper stehen nicht zur verfügung. Das Maximum sind: {}", wallpaper.len() - 1);
    }
    wallpaper[index].clone()
}

pub fn pars_links() -> Config {
    let file: String = format!("{}/links.json", gen_path(3));
    let file_content = fs::read_to_string(&file)
        .expect("Wallpapers.json konnte nicht geparst werden");
    let loaded_config: Config = serde_json::from_str(&file_content)
        .expect("Konnte Wallpapers.json nicht konvertieren");
    loaded_config
}

pub fn pars_rwallpath(index: usize, theme: String) -> String {
    let config = pars_links();
    if let Some(wallpaper_info) = config.theme.get(&theme) {
        if index < wallpaper_info.wallpapers.len() {
            return wallpaper_info.wallpapers[index].clone();
        } else {
            panic!("So viele Wallpaper sind für dieses Theme nicht verlinkt");
        } 
    } else {
        panic!("Dieses Theme wurde nicht gefunden");
    }
}

pub fn pars_themes() -> Vec<String> {
    let folder_path: String = format!("{}/themes", gen_path(2));
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

