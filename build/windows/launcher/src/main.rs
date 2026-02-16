use launcher_api::{LauncherConfig, show_splash, show_logo, show_loading_bar, run_installer, launch_engine};

fn main() {
    let config = LauncherConfig::load("../launcher_config.toml");
    let window_title = std::fs::read_to_string("../launcher_config.toml")
        .ok()
        .and_then(|c| c.lines().find(|l| l.contains("window_title")))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_else(|| "Enchantment Engine Launcher".to_string());
    println!("[TITLE] {}", window_title);
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, &format!("{} - Loading Engine...", window_title));
    if config.enable_installer && !config.dev_mode {
        run_installer(&config);
    }
    launch_engine(&config);
}
