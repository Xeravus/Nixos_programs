use std::env;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct TrackerEntry {
    name: String,
    duration: u64,
    timestamp: u64,
}

fn main() {
    let conn = Connection::open("/home/cato/.config/nix_timetracker/entries.db");
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

    // 3. Verbinde mit dem Unix-Socket
    let stream = UnixStream::connect(&socket_path)
        .expect("Konnte nicht mit dem Hyprland-Socket verbinden!");
    println!("Erfolgreich verbunden! Warte auf Fenster-Wechsel...\n");

    // 4. Lese den Stream Zeile für Zeile (gepuffert für Performance)
    let reader = BufReader::new(stream);
    let mut last_switch_time = Instant::now();
    let mut current_window_class = String::new();

    for line in reader.lines() {
        match line {
            Ok(event) => {
                if event.starts_with("activewindow>>") {
                    let data = event.trim_start_matches("activewindow>>");
                    let parts: Vec<&str> = data.splitn(2, ',').collect();
                    let new_class = if parts.len() >= 1 && !parts[0].is_empty() {
                        parts[0].to_string()
                    } else {
                        "Leerer Workspace".to_string()
                    };

                    let elapsed = last_switch_time.elapsed();
                    let elapsed_seconds = elapsed.as_secs();
                    if !current_window_class.is_empty() && elapsed_seconds > 0 {
                        println!(
                            "Zeit auf das Level-Konto für '{}': {} Sekunden",
                            current_window_class, elapsed_seconds
                        );
                        let start_time = SystemTime::now();
                        let timestamp_now = start_time
                            .duration_since(UNIX_EPOCH)
                            .expect("Systemzeit liegt vor 1970!")
                            .as_secs();

                        // 3. Befülle dein Struct
                        let new_entry = TrackerEntry {
                            name: current_window_class.clone(),
                            duration: elapsed_seconds,
                            timestamp: timestamp_now,
                        };

                        // 4. Speicher das Struct in der Datenbank (die `conn` hast du vorher einmalig geöffnet)
                        // Wir lassen die ID hier weg, SQLite vergibt sie automatisch!
                        let result = conn.execute(
                            "INSERT INTO entry (name, duration, timestamp) VALUES (?1, ?2, ?3)",
                            params![new_entry.name, new_entry.duration, new_entry.timestamp],
                        ).expect("Fehler beim Beschreiben der Datenbank");
                    }
                    current_window_class = new_class.clone();
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
