use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection};
use clap::{Parser, Subcommand};
use serde::Serialize;

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
    }
}

fn run_daemon() {
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
    // 1. Hole die Umgebungsvariablen
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
    println!("Erfolgreich verbunden! Warte auf Fenster-Wechsel...\n");
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
                        println!(
                            "Zeit auf das Level-Konto für '{}': {} Sekunden",
                            current_window_class, elapsed_seconds
                        );
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
                    
                    println!("➡️ Fokus auf: {}", current_window_class);
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
    if class == "kitty" || class == "Alacritty" {
        let title_lower = title.to_lowercase();
        if title_lower.contains("nvim") {
            return "nvim".to_string();
        } else if title_lower.contains("btop") || title_lower.contains("htop") {
            return "system_monitor".to_string();
        } else if title_lower.contains("git") || title_lower.contains("gh") {
            return "git".to_string();
        } else if title_lower.contains("ssh") || title_lower.contains("colmena") {
            return "server_admin".to_string();
        } else if title_lower.contains("nix") || title_lower.contains("nixos") || title_lower.contains("nh") || title_lower.contains("restituo") {
            return "Nixen".to_string();
        } else if title_lower.contains("hashcat") || title_lower.contains("nmap") || title_lower.contains("aircrack-ng") || title_lower.contains("wifite") || title_lower.contains("wireshark") {
            return "Cybersecurity".to_string();
        } else if title_lower.contains("fastfetch") || title_lower.contains("nitch") {
            return "larping".to_string();
        } else {
            return "Terminal".to_string(); 
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
    }).unwrap_or(0); // Falls die Query generell fehlschlägt, ist es auch 0.

    let mut current_level = 0;
    let mut remaining_seconds = total_seconds as f64; 
    let mut seconds_for_next_level: f64 = 0.0;

    loop {
        let hours_required = 1.0 + 2.0 * 1.15_f64.powi(current_level);
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
