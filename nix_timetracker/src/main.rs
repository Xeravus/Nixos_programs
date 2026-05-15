use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::fs::File;
use fs3::FileExt;
use notify_rust::Notification;

#[derive(Parser)]
#[command(name = "nix-timetracker")]
#[command(about = "Trackt Fensterzeiten und berechnet RPG-Level", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Daemon,
    Status {
        app: String,
    },
    Listapps,
    Statusall,
}

#[derive(Serialize)]
struct AppStatus {
    app: String,
    level: i64,
    progress_percent: u8,
    total_seconds: i64,
}

#[derive(Debug)]
struct TrackerEntry {
    name: String,
    duration: i64,
    timestamp: i64,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Daemon => {
            println!("Starte Nix-Timetracker als Daemon");
            run_daemon();
        }
        Commands::Status { app } => {
            get_status(app);
        }
        Commands::Listapps => {
            list_apps();
        }
        Commands::Statusall => {
            status_all();
        }
    }
}

fn notify_user(title: &str, message: &str) {
    let fulltitle: String = format!("Nix-Timetracker: {}", title);
    let fullmessage: String = format!("Nix-Timetracker {}", message);
    let _ = Notification::new()
        .summary(&fulltitle)
        .body(&fullmessage)
        .appname("Nix-Timetracker")
        .timeout(5000)
        .show();
}

fn run_daemon() {
    let lock_file = File::create("/home/cato/.config/nix_timetracker/daemon.lock")
        .expect("Konnte Lock-Datei nicht erstellen!");
    if lock_file.try_lock_exclusive().is_err() {
        eprintln!("Abbruch: Der Timetracker-Daemon läuft bereits im Hintergrund!");
        std::process::exit(1); 
    }
    notify_user("Started Daemon", "started daemon");
    let conn = Connection::open("/home/cato/.config/nix_timetracker/entries.db")
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

fn normalize_app_name(class: &str, title: &str) -> String {
    if class == "kitty" || class == "kitty-floating" || class == "Alacritty" {
        let title_lower = title.to_lowercase();
        if title_lower.contains("rust") || title_lower.contains("rs") {
            return "rust".to_string();
        } else if title_lower.contains("nix") || title_lower.contains("nixos") || title_lower.contains("nh") || title_lower.contains("restituo") {
            return "nixen".to_string();
        } else if title_lower.contains("nvim") || title_lower == "v" || title_lower == "sv" || title_lower.contains("vim") {
            return "nvim".to_string();
        } else if title_lower.contains("btop") || title_lower.contains("htop") {
            return "system_monitor".to_string();
        } else if title_lower.contains("git") || title_lower.contains("gh") {
            return "git".to_string();
        } else if title_lower.contains("ssh") || title_lower.contains("colmena") {
            return "server_admin".to_string();
        } else if title_lower.contains("hashcat") || title_lower.contains("nmap") || title_lower.contains("aircrack-ng") || title_lower.contains("wifite") || title_lower.contains("wireshark") {
            return "cybersecurity".to_string();
        } else if title_lower.contains("fastfetch") || title_lower.contains("nitch") {
            return "larping".to_string();
        } else {
            return "terminal".to_string(); 
        }
    } else if class == "zen-beta" || class == "firefox" {
        let title_lower = title.to_lowercase();
        if title_lower.contains("youtube") {
            return "procrastination".to_string();
        } else if title_lower.contains("chatgpt") || title_lower.contains("gemini") || title_lower.contains("claude") {
            return "llm".to_string();
        } else if title_lower.contains("rust") {
            return "rust".to_string();
        } else if title_lower.contains("nix") || title_lower.contains("nixos") {
            return "nixen".to_string();
        } else if title_lower.contains("git") || title_lower.contains("github") {
            return "git".to_string();
        } else {
            return "browser".to_string();
        }
    }
    class.to_string()
}

fn get_status(app_name: &str) {
    let conn = rusqlite::Connection::open("/home/cato/.config/nix_timetracker/entries.db")
        .expect("Konnte Datenbank für Status-Abfrage nicht öffnen!");
    let mut stmt = conn.prepare("SELECT SUM(duration) FROM entry WHERE name = ?1")
        .expect("Konnte SQL-Query nicht vorbereiten!");
    let total_seconds: i64 = stmt.query_row(rusqlite::params![app_name], |row| {
        let val: Option<i64> = row.get(0)?;
        Ok(val.unwrap_or(0))
    }).unwrap_or(0); 

    let mut current_level = 0;
    let mut remaining_seconds = total_seconds as f64; 
    let mut seconds_for_next_level: f64 = 0.0;

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
    let status = AppStatus {
        app: app_name.to_string(),
        level: current_level as i64,      
        progress_percent,
        total_seconds,
    };
    let json_output = serde_json::to_string(&status)
        .expect("Konnte Struct nicht in JSON umwandeln!");
    println!("{}", json_output);
}

fn list_apps() {
    let conn = Connection::open("/home/cato/.config/nix_timetracker/entries.db")
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

fn status_all() {
    let conn = Connection::open("/home/cato/.config/nix_timetracker/entries.db")
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
    for i in apps {
        get_status(&i);
    }
}
