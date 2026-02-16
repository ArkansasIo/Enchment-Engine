//! API for installer app (modular, reusable)

use std::fs;
use toml::Value;

pub struct InstallerConfig {
    pub splash_screen: String,
    pub logo: String,
    pub loading_bar_color: String,
    pub loading_bar_background: String,
}

impl InstallerConfig {
    pub fn load(path: &str) -> Self {
        let config = fs::read_to_string(path).expect("Failed to read config");
        let value: Value = toml::from_str(&config).expect("Invalid TOML");
        Self {
            splash_screen: value["splash_screen"].as_str().unwrap_or("").to_string(),
            logo: value["logo"].as_str().unwrap_or("").to_string(),
            loading_bar_color: value["loading_bar_color"].as_str().unwrap_or("#FFD740").to_string(),
            loading_bar_background: value["loading_bar_background"].as_str().unwrap_or("#222222").to_string(),
        }
    }
}

pub fn show_splash(config: &InstallerConfig) {
    println!("[INSTALLER SPLASH] {}", config.splash_screen);
}

pub fn show_logo(config: &InstallerConfig) {
    println!("[INSTALLER LOGO] {}", config.logo);
}

pub fn show_loading_bar(config: &InstallerConfig, msg: &str) {
    println!("[INSTALLER LOADING] {} (bar: {}, bg: {})", msg, config.loading_bar_color, config.loading_bar_background);
    for i in 0..=100 {
        print!("\r[INSTALLER] {}%", i);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    println!("\n[INSTALLER] Complete!");
}

pub fn perform_install(_config: &InstallerConfig) {
    println!("[INSTALLER] Performing installation steps...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("[INSTALLER] Installation finished!");
}
