use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs;
use std::env;
use std::collections::HashMap;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "nix-switcher")]
#[command(about = "Verwaltet Theming und Wallpaper von Hyprland, Hyprpaper, Quickshell, etc.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set {
        theme: String,
        wallpaper_index: usize,
    },
    Apply,
    Init,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    theme: String,
    themedir: String,
    wallpaper: String,
    wallpaperdir: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    theme: HashMap<String, Wallpapers>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wallpapers {
    wallpapers: Vec<String>,
}

fn gen_path(option: u8) -> String {
    let home = env::var("HOME").expect("Konnte die Hoemvariable nicht finden");
    let option1: String = format!("{}/.config/rice", home);
    let option2: String = format!("{}/nix-switcher", option1);
    if option == 1 {
        return home
    } else if option == 2 {
        return option1
    } else if option == 3 {
        return option2
    } else {
        panic!("Keine Option");
    }
}

fn gen_file_config() {
    let themedir = format!("{}/themes", gen_path(2));
    let wallpaperdir = format!("{}/wallpaper", gen_path(2));
    let mut theme_map: HashMap<String, Wallpapers> = HashMap::new();
    let basic_conf = Data {
        theme: String::from("dracula"),
        themedir: themedir,
        wallpaper: String::from("dracula"),
        wallpaperdir: wallpaperdir,
    };
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
    let json_string = serde_json::to_string_pretty(&basic_conf).unwrap();
    let json_path: String = format!("{}/config.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
    let json_string_wallpaper = serde_json::to_string_pretty(&config_out).unwrap();
    let json_path_wallpaper: String = format!("{}/links.json", gen_path(3));
    fs::write(&json_path_wallpaper, &json_string_wallpaper).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path_wallpaper);
}

fn change(theme: &str, wallpaper_index: usize) {
    let structin = pars_config();
    let structout = Data {
        theme: String::from(theme),
        wallpaper: wallpath(wallpaper_index),
        ..structin
    };
    let json_string = serde_json::to_string_pretty(&structout).unwrap();
    let json_path: String = format!("{}/config.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
}

fn apply(structin: Data) {
    let wallpapercommand: String = format!(",{}", structin.wallpaper);
    replace_pointer(&structin);
    Command::new("hyprctl")
        .args(["hyprpaper", "preload", &structin.wallpaper])
        .status()
        .expect("Konnte das Wallpaper nicht preloaden");
    Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &wallpapercommand])
        .status()
        .expect("Konnte das Wallpaper nicht ändern");
    Command::new("hyprctl")
        .args(["hyprpaper", "unload", "unused"])
        .status()
        .expect("Konnte unbenutzte Wallpaper nicht entbinden");
    Command::new("killall")
        .args(["-SIGUSR1", ".kitty-wrapped"])
        .status()
        .expect("Konnte Kitty nicht neuladen");
}
/*
fn link_theme_wallpaper(wallpaperindex: usize, themes: Vec<String>) {
    let mut config = pars_config();
    let target_wallpaper: String = wallpath(wallpaperindex);
    for i in themes {
        if let Some(wallpaper_info) = config.wallpaper.get_mut(&target_wallpaper) {
            let new_theme = i.to_string();
            if !wallpaper_info.themes.contains(&new_theme) {
                wallpaper_info.themes.push(new_theme);
            } else {
                println!("Das Theme({}) ist schon mit dem Theme({}) verknüpft", &target_wallpaper, &new_theme);
            }
        } else {
            println!("Konnte das Wallpaper({}) in wallpapers.json nicht finden", &target_wallpaper);
            return;
        }
    }
    let file_path: String = format!("{}/wallpapers.json", gen_path(2));
    let json_string = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&file_path, &json_string).expect("Konnte wallpapers.json nicht überschreiben");
}
*/

fn replace_pointer(theme: &Data) {
    let themedir_path: String = pars_config().themedir;
    let hyprland_path: String = format!("{}/{}/hyprland/color.conf", &themedir_path, &theme.theme);
    let hyprland_base: String = format!("{}/.config/hypr/color.conf", gen_path(1));
    let rofi_path: String = format!("{}/{}/rofi/current.rasi", &themedir_path, &theme.theme);
    let rofi_base: String = format!("{}/.config/rofi/current.rasi", gen_path(1));
    let kitty_path: String = format!("{}/{}/kitty/current.conf", &themedir_path, &theme.theme);
    let kitty_base: String = format!("{}/.config/kitty/current.conf", gen_path(1));
    let quickshell_path: String = format!("{}/{}/quickshell/current.qml", &themedir_path, &theme.theme);
    let quickshell_base: String = format!("{}/.config/quickshell/color/current.qml", gen_path(1));
    let quickshell_touch: String = format!("{}/.config/quickshell/shell.qml", gen_path(1));
    Command::new("cp")
        .args([&hyprland_path, &hyprland_base])
        .status()
        .expect("Konnte den Hyprland Pointer nicht ersetzten");
    Command::new("cp")
        .args([&rofi_path, &rofi_base])
        .status()
        .expect("Konnte den Rofi Pointer nicht ersetzten");
    Command::new("cp")
        .args([&kitty_path, &kitty_base])
        .status()
        .expect("Konnte den Kitty Pointer nicht ersetzten");
    Command::new("cp")
        .args([&quickshell_path, &quickshell_base])
        .status()
        .expect("Konnte den Quickshell Pointer nicht ersetzten");
    Command::new("touch")
        .arg(&quickshell_touch)
        .status()
        .expect("Konnte keinen neuen Zeitstempel in der Shell.qml erstellen");
}

fn pars_config() -> Data {
    let json_path: String = format!("{}/config.json", gen_path(3));
    let file_content = fs::read_to_string(&json_path).expect("Datei konnte nicht gelesen werden");
    let loaded_config: Data = serde_json::from_str(&file_content).unwrap();
    loaded_config
}

fn pars_wall() -> Vec<String> {
    let folder_path: String = format!("{}/wallpaper", gen_path(2));
    WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

fn wallpath(index: usize) -> String {
    let wallpaper = pars_wall();
    if index >=wallpaper.len() {
        panic!("So viele Wallpaper stehen nicht zur verfügung. Das Maximum sind: {}", wallpaper.len() - 1);
    }
    wallpaper[index].clone()
}

fn pars_themes() -> Vec<String> {
    let folder_path: String = format!("{}/themes", gen_path(2));
    WalkDir::new(&folder_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter_map(|entry| {
            entry.path().file_name().and_then(|n| n.to_str()).map(|s| s.to_string())
        })
    .collect()
}

fn pars_links() -> Config {
    let file: String = format!("{}/links.json", gen_path(3));
    let file_content = fs::read_to_string(&file)
        .expect("Wallpapers.json konnte nicht geparst werden");
    let loaded_config: Config = serde_json::from_str(&file_content)
        .expect("Konnte Wallpapers.json nicht konvertieren");
    loaded_config
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
       Commands::Set { theme, wallpaper_index } => {
           change(&theme, *wallpaper_index);
       }
       Commands::Apply => {
           let config = pars_config();
           apply(config);
       }
       Commands::Init => {
           gen_file_config();
           println!("Generated Basic Config");
       }
    }
}
