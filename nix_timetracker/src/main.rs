use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveWindow;
use hyprland::Result;

fn main() -> Result<()> {
    // Erstelle einen neuen Event-Listener
    let mut listener = EventListener::new();

    // Reagiere auf den Wechsel des aktiven Fensters
    listener.add_active_window_change_handler(|window_data| {
        if let Some(window) = window_data {
            // window_class ist z.B. "kitty", "firefox" oder "neovim"
            let class = window.window_class;
            let title = window.window_title;
            
            println!("Fokus auf: {} (Titel: {})", class, title);
            
            // HIER kommt später deine Logik rein:
            // 1. Stoppe die Zeit des VORHERIGEN Fensters.
            // 2. Speichere die Zeit ab.
            // 3. Starte den Timer für DIESES neue Fenster.
        } else {
            println!("Kein aktives Fenster (z.B. auf einen leeren Workspace gewechselt)");
        }
    });

    println!("Tracker gestartet! Warte auf Fenster-Wechsel...");
    
    // Starte den Listener (blockiert den Thread, damit das Programm weiterläuft)
    listener.start_listener()
}
