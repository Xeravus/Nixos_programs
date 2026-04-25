use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use std::fs;
use std::env;
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
enum Commands{
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

fn gen_path() -> String {
    let home = env::var("HOME").expect("Konnte die Homevariable nicht finden");
    format!("{}/.config/rice", home)
}

fn gen_home() -> String {
    let home = env::var("HOME").expect("Konnte die Homevariable nicht finden");
    home
}

fn gen_config() {
    let themedir = format!("{}/themes", gen_path());
    let wallpaperdir = format!("{}/wallpaper", gen_path());
    let basic_conf = Data {
        theme: String::from("dracula"),
        themedir: themedir,
        wallpaper: String::from("dracula"),
        wallpaperdir: wallpaperdir,
    };
    let json_string = serde_json::to_string_pretty(&basic_conf).unwrap();
    let json_path: String = format!("{}/switcher.json", gen_path());
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
}

fn gen_walllist() {
    let json_string = serde_json::to_string_pretty(&wallpars()).unwrap();
    let json_path: String = format!("{}/wallpapers.json", gen_path());
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
} 

fn change(theme: &str, wallpaper_index: usize) {
    let structin = read();
    let structout = Data {
        theme: String::from(theme),
        wallpaper: wallpath(wallpaper_index),
        ..structin
    };
    let json_string = serde_json::to_string_pretty(&structout).unwrap();
    let json_path: String = format!("{}/switcher.json", gen_path());
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

fn replace_pointer(theme: &Data) {
    let themedir_path: String = read().themedir;
    let hyprland_path: String = format!("{}/{}/hyprland/color.conf", &themedir_path, &theme.theme);
    let hyprland_base: String = format!("{}/.config/hypr/color.conf", gen_home());
    let rofi_path: String = format!("{}/{}/rofi/current.rasi", &themedir_path, &theme.theme);
    let rofi_base: String = format!("{}/.config/rofi/current.rasi", gen_home());
    let kitty_path: String = format!("{}/{}/kitty/current.conf", &themedir_path, &theme.theme);
    let kitty_base: String = format!("{}/.config/kitty/current.conf", gen_home());
    let quickshell_path: String = format!("{}/{}/quickshell/current.qml", &themedir_path, &theme.theme);
    let quickshell_base: String = format!("{}/.config/quickshell/color/current.qml", gen_home());
    let quickshell_touch: String = format!("{}/.config/quickshell/shell.qml", gen_home());
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

fn read() -> Data {
    let json_path: String = format!("{}/switcher.json", gen_path());
    let file_content = fs::read_to_string(&json_path).expect("Datei konnte nicht gelesen werden");
    let loaded_config: Data = serde_json::from_str(&file_content).unwrap();
    loaded_config
}

fn wallpars() -> Vec<String> {
    let folder_path: String = format!("{}/wallpaper", gen_path());
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
    let wallpaper = wallpars();
    if index >=wallpaper.len() {
        panic!("So viele Wallpaper stehen nicht zur verfügung. Das Maximum sind: {}", wallpaper.len() - 1);
    }
    wallpaper[index].clone()
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
       Commands::Set { theme, wallpaper_index } => {
           change(&theme, *wallpaper_index);
       }
       Commands::Apply => {
           let config = read();
           apply(config);
       }
       Commands::Init => {
           gen_config();
           gen_walllist();
           println!("Generated Basic Config");
       }
    }
}
