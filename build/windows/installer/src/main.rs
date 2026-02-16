use launcher_api::{LauncherConfig, show_splash, show_logo, show_loading_bar};

fn main() {
    let config = LauncherConfig::load("../launcher_config.toml");
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, "Installing...");
    println!("[INSTALLER] Installation steps would go here.");
}
