use launcher_api::{LauncherConfig, show_splash, show_logo, show_loading_bar};

fn main() {
    let config = LauncherConfig::load("../launcher_config.toml");
    let installer_title = std::fs::read_to_string("../launcher_config.toml")
        .ok()
        .and_then(|c| c.lines().find(|l| l.contains("installer_title")))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_else(|| "Enchantment Engine Installer".to_string());
    println!("[TITLE] {}", installer_title);
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, &format!("{} - Installing...", installer_title));
    println!("[INSTALLER] Installation steps would go here.");
}
