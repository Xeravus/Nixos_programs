#![allow(unused_imports)]
mod parsers;
mod generator;
mod set;
mod clean;

use parsers::*;
use generator::*;
use set::*;
use clean::*;

use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;
use std::path::*; 
use notify_rust::Notification;

#[derive(Parser)]
#[command(name = "nix-switcher")]
#[command(about = "Verwaltet Theming und Wallpaper von Hyprland, Hyprpaper, Quickshell, etc.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setg {
        theme: String,
        index: usize,
    },
    Set {
        theme: String,
        index: usize,
    },
    Settheme {
        theme: String,
    },
    Setwall {
        index: usize,
    },
    Setkitty {
        theme: String,
    },
    Reload,
    Genthemes,
    Cleanthemes, 
    Apply,
    Init,
    Genwall,
    Link {
        index: usize,
        theme: Vec<String>,
    },
}

pub fn notify_user(title: &str, message: &str) {
    let fulltitle: String = format!("Nix-Switcher: {}", title);
    let fullmessage: String = format!("Nix-Switcher {}", message);
    let _ = Notification::new()
        .summary(&fulltitle)
        .body(&fullmessage)
        .appname("Nix-Switcher")
        .timeout(5000)
        .show();
}

pub fn apply(structin: Data) {
    replace_pointer(&structin);
    Command::new("swww")
        .args(["img", &structin.wallpaper, "--transition-type", "grow", "--transition-pos", "0.5,0.5", "--transition-step", "90", "--transition-fps", "60"])
        .spawn()
        .expect("SWWW Hat es nicht auf die Reihe bekommen");
    replace_kitty();
    Command::new("killall")
        .args(["-SIGUSR1", ".kitty-wrapped"])
        .status()
        .expect("Konnte Kitty nicht neuladen");
    notify_user("Apply", "apply");
}

pub fn replace_pointer(theme: &Data) {
    let hyprland_path: String = PathBuf::from(gen_path(PathType::Themes)).join(&theme.theme).join("hyprland_template.conf").to_str().unwrap().to_string();
    let hyprland_base: String = PathBuf::from(gen_path(PathType::Config)).join("hypr").join("color.conf").to_str().unwrap().to_string();
    let quickshell_path: String = PathBuf::from(gen_path(PathType::Themes)).join(&theme.theme).join("quickshell_template.qml").to_str().unwrap().to_string();
    let quickshell_base: String = PathBuf::from(gen_path(PathType::Config)).join("quickshell").join("color").join("Current.qml").to_str().unwrap().to_string();
    let quickshell_touch: String = PathBuf::from(gen_path(PathType::Config)).join("quickshell").join("shell.qml").to_str().unwrap().to_string();
    fs::copy(&hyprland_path, &hyprland_base).expect("Konnte den Hyprland Pointer nicht ersetzen");
    fs::copy(&quickshell_path, &quickshell_base).expect("Konnte den Quickshell Pointer nicht ersetzen");
    Command::new("touch")
        .arg(&quickshell_touch)
        .spawn()
        .expect("Quickshell konnte nicht getoucht werden");
}

pub fn replace_kitty() {
    let themedir_path: String = pars_config().kittytheme;
    let kitty_path: String = format!("{}/{}.conf", gen_path(PathType::Kittythemes), &themedir_path);
    let kitty_base = PathBuf::from(gen_path(PathType::Config)).join("kitty").join("current.conf").to_str().unwrap().to_string();
    fs::copy(&kitty_path, &kitty_base).expect("Konnte Kitty nicht austauschen");
}

pub fn link_theme_wallpaper(wallpaperindex: usize, themes: Vec<String>) {
    let mut config = pars_links();
    let target_wallpaper: String = pars_gwallpath(wallpaperindex);
    for i in themes {
        if let Some(wallpaper_info) = config.theme.get_mut(&i) {
            if let Some(index) = wallpaper_info.wallpapers.iter().position(|x| *x == target_wallpaper) {
                wallpaper_info.wallpapers.remove(index); 
                notify_user("Wallpaper unlinked", "unlinked Wallpaper to Theme");
            } else {
                wallpaper_info.wallpapers.push(target_wallpaper.clone());
                println!("Erfolg: Wallpaper wurde mit '{}' verknüpft!", i);
                notify_user("Wallpaper linked", "linked Wallpaper to Theme");
            }
        } else {
            println!("Fehler: Das Theme '{}' existiert in der config nicht.", i);
        }
    }
    let file_path = PathBuf::from(gen_path(PathType::Nixswitcher)).join("links.json").to_str().unwrap().to_string();
    let json_string = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&file_path, &json_string).expect("Konnte wallpapers.json nicht überschreiben");
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            gen_file_init();
            println!("Generated Basic Config");
            notify_user("Gen base Config", "generated basic Config");
        }
        Commands::Genwall => {
            gen_file_wallpaper();
        }
        Commands::Link { index, theme } => {
            link_theme_wallpaper(*index, theme.clone());
        }
        Commands::Setg { theme, index } => {
            change(set_global(&theme, *index));
        }
        Commands::Set { theme, index } => {
            change(set_relativ(&theme, *index));
        }
        Commands::Settheme { theme } => {
            change(set_theme(&theme));
            change(set_relativ(&theme, 0));
        }
        Commands::Setwall { index } => {
            change(set_wall(*index));
        }
        Commands::Setkitty { theme } => {
            change(set_kittytheme((&theme).to_string()));
        }
        Commands::Reload => {
            cl_themedir();
            gen_themes_all();
            gen_file_wallpaper();
            update_file_links();
        }
        Commands::Genthemes => {
            gen_themes_all();
        }
        Commands::Cleanthemes => {
            cl_themedir();
        }
        Commands::Apply => {
            let config = pars_config();
            apply(config);
        }
    }
}
