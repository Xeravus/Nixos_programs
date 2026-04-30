mod parsers;
mod generator;
mod set;

use parsers::*;
use generator::*;
use set::*;

use clap::{Parser, Subcommand};
use std::fs;
use std::fs::OpenOptions;
use std::process::Command;


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
    Apply,
    Init,
    Link {
        index: usize,
        theme: Vec<String>,
    },
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

fn link_theme_wallpaper(wallpaperindex: usize, themes: Vec<String>) {
    let mut config = pars_links();
    let target_wallpaper: String = pars_gwallpath(wallpaperindex);
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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            gen_file_init();
            println!("Generated Basic Config");
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
        }
        Commands::Setwall { index } => {
            change(set_wall(*index));
        }
        Commands::Apply => {
            let config = pars_config();
            apply(config);
        }
    }
}
