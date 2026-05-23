#![allow(unused_imports)]
mod formater;
mod norm;

use formater::*;
use norm::*;

use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use rusqlite::{params, Connection};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::fs::File;
use fs3::FileExt;
use notify_rust::Notification;
use humantime::format_duration;

#[derive(Parser)]
#[command(name = "nix-timetracker")]
#[command(about = "Trackt Fensterzeiten und berechnet RPG-Level", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Daemon,
    Status {
        app: String,
        #[arg(short = 'j', long = "json", conflicts_with = "readable")]
        json: bool,
        #[arg(short = 'r', long = "readable")]
        readable: bool,
        #[arg(short = 'c', long = "compact", conflicts_with = "extended")]
        compact: bool,
        #[arg(short = 'e', long = "extended", conflicts_with = "compact")]
        extended: bool,
    },
    Listapps,
    Statusall {
        #[arg(short = 'j', long = "json", conflicts_with = "readable")]
        json: bool,
        #[arg(short = 'r', long = "readable")]
        readable: bool,
        #[arg(short = 'c', long = "compact", conflicts_with = "extended")]
        compact: bool,
        #[arg(short = 'e', long = "extended", conflicts_with = "compact")]
        extended: bool,
    },
}

#[derive(Serialize)]
pub struct AppStatusJson {
    app: String,
    level: i64,
    progress_percent: u8,
    total_seconds: u64,
}

#[derive(Serialize, Debug)]
pub struct AppStatusReadable {
    app: String,
    level: i64,
    progress_percent: u8,
    time: String,
}

#[derive(Debug)]
pub struct TrackerEntry {
    name: String,
    duration: i64,
    timestamp: i64,
}

pub fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Daemon => {
            println!("Starte Nix-Timetracker als Daemon");
            run_daemon();
        }
        Commands::Status { app, json, readable, compact, extended } => {
            get_status(app, &take_input(*json, *readable, *compact, *extended), None);
        }
        Commands::Listapps => {
            list_apps();
        }
        Commands::Statusall { json, readable, compact, extended } => {
            get_status_all(take_input(*json, *readable, *compact, *extended));
        }
    }
}

pub fn notify_user(title: &str, message: &str) {
    let fulltitle: String = format!("Nix-Timetracker: {}", title);
    let fullmessage: String = format!("Nix-Timetracker {}", message);
    let _ = Notification::new()
        .summary(&fulltitle)
        .body(&fullmessage)
        .appname("Nix-Timetracker")
        .timeout(5000)
        .show();
}

pub fn run_daemon() {
    let lock_file = File::create("/home/cato/.config/nix-timetracker/daemon.lock")
        .expect("Konnte Lock-Datei nicht erstellen!");
    if lock_file.try_lock_exclusive().is_err() {
        eprintln!("Abbruch: Der Timetracker-Daemon läuft bereits im Hintergrund!");
        std::process::exit(1); 
    }
    notify_user("Started Daemon", "started daemon");
    let conn = Connection::open("/home/cato/.config/nix-timetracker/entries.db")
        .expect("Konnte Datenbank nicht öffnen! Existiert der Ordner?");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entry (
            id          INTEGER     PRIMARY     KEY,
            name        TEXT        NOT         NULL,
            duration    INTEGER,
            timestamp   INTEGER
        )",
        (),
    ).expect("Fehler beim verbinden der Datenbank");
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")
        .expect("Konnte XDG_RUNTIME_DIR nicht finden!");
    let hyprland_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Konnte HYPRLAND_INSTANCE_SIGNATURE nicht finden! Läuft Hyprland?");

    // 2. Baue den Pfad zum Event-Socket zusammen
    let mut socket_path = PathBuf::from(xdg_runtime_dir);
    socket_path.push("hypr");
    socket_path.push(hyprland_sig);
    socket_path.push(".socket2.sock");
    println!("Versuche Verbindung zu: {:?}", socket_path);
    let stream = UnixStream::connect(&socket_path)
        .expect("Konnte nicht mit dem Hyprland-Socket verbinden!");
    let reader = BufReader::new(stream);
    let mut last_switch_time = Instant::now();
    let mut current_window_class = String::new();

    for line in reader.lines() {
        match line {
            Ok(event) => {
                if event.starts_with("activewindow>>") {
                    let data = event.trim_start_matches("activewindow>>");
                    let parts: Vec<&str> = data.splitn(2, ',').collect();

                    let class = if parts.len() >= 1 { parts[0] } else { "" };
                    let title = if parts.len() == 2 { parts[1] } else { "" };

                    // 1. Nutze unsere neue Funktion, um den echten Namen zu bestimmen
                    let new_app = if class.is_empty() {
                        "Leerer Workspace".to_string()
                    } else {
                        normalize_app_name(class, title)
                    };
                    if new_app == current_window_class {
                        continue; 
                    }

                    let elapsed = last_switch_time.elapsed();
                    let elapsed_seconds = elapsed.as_secs() as i64;
                    if !current_window_class.is_empty() && elapsed_seconds > 0 {
                        let start_time = SystemTime::now();
                        let timestamp_now = start_time
                            .duration_since(UNIX_EPOCH)
                            .expect("Systemzeit liegt vor 1970!")
                            .as_secs() as i64;
                        let new_entry = TrackerEntry {
                            name: current_window_class.clone(),
                            duration: elapsed_seconds,
                            timestamp: timestamp_now,
                        };
                        let _ = conn.execute(
                            "INSERT INTO entry (name, duration, timestamp) VALUES (?1, ?2, ?3)",
                            params![new_entry.name, new_entry.duration, new_entry.timestamp],
                        ).expect("Fehler beim Beschreiben der Datenbank");
                    }
                    current_window_class = new_app.clone();
                    last_switch_time = Instant::now(); // Timer neu starten
                }
            }
            Err(e) => {
                eprintln!("Verbindungsabbruch: {}", e);
                break;
            }
        }
    }
}

pub fn get_status(app_name: &str, format: &Format, color_index: Option<usize>) {
    let conn = rusqlite::Connection::open("/home/cato/.config/nix-timetracker/entries.db")
        .expect("Konnte Datenbank für Status-Abfrage nicht öffnen!");
    let mut stmt = conn.prepare("SELECT SUM(duration) FROM entry WHERE name = ?1")
        .expect("Konnte SQL-Query nicht vorbereiten!");
    let total_seconds: u64 = stmt.query_row(rusqlite::params![app_name], |row| {
        let val: Option<i64> = row.get(0)?;
        Ok(val.unwrap_or(0))
    }).unwrap_or(0).try_into().unwrap(); 

    let mut current_level = 0;
    let mut remaining_seconds = total_seconds as f64; 
    let mut seconds_for_next_level: f64;

    loop {
        let hours_required = 2.0 * 1.15_f64.powi(current_level);
        seconds_for_next_level = hours_required * 3600.0;
        if remaining_seconds >= seconds_for_next_level {
            remaining_seconds -= seconds_for_next_level;
            current_level += 1;
        } else {
            break; 
        }
    }

    let progress_percent = ((remaining_seconds / seconds_for_next_level) * 100.0).round() as u8;
    let status = AppStatusJson {
        app: app_name.to_string(),
        level: current_level as i64,      
        progress_percent,
        total_seconds,
    };
    format_output(status, format, color_index);
}

pub fn list_apps() {
    let conn = Connection::open("/home/cato/.config/nix-timetracker/entries.db")
        .expect("Konnte DB nicht öffnen");
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM entry ORDER BY name ASC")
        .expect("Fehler beim Vorbereiten der Query");
    let app_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        Ok(name)
    }).expect("Fehler beim Mapping der Daten");
    let apps: Vec<String> = app_iter
        .filter_map(|res| res.ok()) 
        .collect();
    let json_output = serde_json::to_string(&apps)
        .expect("Konnte Liste nicht in JSON umwandeln");
    println!("{}", json_output);
}

pub fn get_status_all(format: Format) {
    let conn = Connection::open("/home/cato/.config/nix-timetracker/entries.db")
        .expect("Konnte DB nicht öffnen");
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM entry ORDER BY name ASC")
        .expect("Fehler beim Vorbereiten der Query");
    let app_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        Ok(name)
    }).expect("Fehler beim Mapping der Daten");
    let apps: Vec<String> = app_iter
        .filter_map(|res| res.ok()) 
        .collect();
    for (index, i) in apps.iter().enumerate() {
        get_status(&i, &format, Some(index));
    }
}

