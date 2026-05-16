pub fn normalize_app_name(class: &str, title: &str) -> String {
    if class == "kitty" || class == "kitty-floating" || class == "Alacritty" {
        let title_lower = title.to_lowercase();
        if title_lower.contains("nvim") || title_lower == "v" || title_lower == "sv" || title_lower.contains("vim") {
            return "nvim".to_string();
        } else if title_lower.contains("btop") || title_lower.contains("htop") {
            return "system_monitor".to_string();
        } else if title_lower.contains("git") || title_lower.contains("gh") {
            return "git".to_string();
        } else if title_lower.contains("ssh") || title_lower.contains("colmena") {
            return "server_admin".to_string();
        } else if title_lower.contains("nix") || title_lower.contains("nixos") || title_lower.contains("nh") || title_lower.contains("restituo") {
            return "nixen".to_string();
        } else if title_lower.contains("hashcat") || title_lower.contains("nmap") || title_lower.contains("aircrack-ng") || title_lower.contains("wifite") || title_lower.contains("wireshark") {
            return "cybersecurity".to_string();
        } else if title_lower.contains("fastfetch") || title_lower.contains("nitch") {
            return "larping".to_string();
        } else {
            return "terminal".to_string(); 
        }
    } else if class == "zen-beta" || class == "firefox" {
        let title_lower = title.to_lowercase();
        if title_lower.contains("nix") || title_lower.contains("nixos") {
            return "nixen".to_string();
        } else if title_lower.contains("youtube") {
            return "procrastination".to_string();
        } else if title_lower.contains("chatgpt") || title_lower.contains("gemini") || title_lower.contains("claude") {
            return "llm".to_string();
        } else if title_lower.contains("git") || title_lower.contains("github") {
            return "git".to_string();
        } else {
            return "browser".to_string();
        }
    }
    class.to_string()
}
