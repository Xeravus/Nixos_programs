use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs;
use std::fs::OpenOptions;
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
    Link {
        wallpaper_index: usize,
        theme: Vec<String>,
    },
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

fn gen_file_init() {
    gen_file_config();
    gen_file_links();
    gen_file_wallpaper();
}

fn gen_file_config() {
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

fn gen_file_links() {
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


fn gen_file_wallpaper() {
    let json_string = serde_json::to_string_pretty(&pars_wall()).unwrap();
    let json_path: String = format!("{}/wallpaper.json", gen_path(3));
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    println!("New File in: {}", &json_path);
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
fn link_theme_wallpaper(wallpaperindex: usize, themes: Vec<String>) {
    let mut config = pars_links();
    let target_wallpaper: String = wallpath(wallpaperindex);
    for i in themes {
        if let Some(wallpaper_info) = config.theme.get_mut(&i) {
            if wallpaper_info.wallpapers.contains(&target_wallpaper) {
                println!("Info: Wallpaper ist schon im Theme '{}' verlinkt.", i);
            } else {
                wallpaper_info.wallpapers.push(target_wallpaper.clone());
                println!("Erfolg: Wallpaper wurde mit '{}' verknüpft!", i);
            }
        } else {
            println!("Fehler: Das Theme '{}' existiert in der config nicht.", i);
        }
    }
    let file_path: String = format!("{}/links.json", gen_path(3));
    let json_string = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&file_path, &json_string).expect("Konnte wallpapers.json nicht überschreiben");
}

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
    fs::copy(&hyprland_path, &hyprland_base).expect("Konnte den Hyprland Pointer nicht ersetzen");
    fs::copy(&rofi_path, &rofi_base).expect("Konnte den Rofi Pointer nicht ersetzen");
    fs::copy(&kitty_path, &kitty_base).expect("Konnte den Kitty Pointer nicht ersetzen");
    fs::copy(&quickshell_path, &quickshell_base).expect("Konnte den Quickshell Pointer nicht ersetzen");
    OpenOptions::new()
        .write(true)
        .open(&quickshell_touch)
        .expect("Quickshell konnte nicht getoucht werden");
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
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect()
}

fn pars_wallfile() -> Vec<String> {
    let file: String = format!("{}/wallpaper.json", gen_path(3));
    let file_content = fs::read_to_string(&file).expect("Konnte Wallpaper.json nicht parsen");
    let loaded_content: Vec<String> = serde_json::from_str(&file_content).expect("Konnte Wallpaper.json nicht formatieren");
    loaded_content
}

fn wallpath(index: usize) -> String {
    let wallpaper = pars_wallfile();
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
           gen_file_init();
           println!("Generated Basic Config");
       }
       Commands::Link { wallpaper_index, theme } => {
           link_theme_wallpaper(*wallpaper_index, theme.clone());
       }
    }
}
