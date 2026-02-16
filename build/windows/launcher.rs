//! Simple launcher for Enchantment Engine with splash, logo, loading bar, and config-driven installer toggle

use std::{fs, thread, time::Duration};
use std::path::Path;
use toml::Value;

fn main() {
    // Load config
    let config_path = "launcher_config.toml";
    let config = fs::read_to_string(config_path).expect("Failed to read launcher_config.toml");
    let config: Value = toml::from_str(&config).expect("Invalid TOML in launcher_config.toml");

    let enable_installer = config["enable_installer"].as_bool().unwrap_or(true);
    let dev_mode = config["dev_mode"].as_bool().unwrap_or(false);
    let splash_screen = config["splash_screen"].as_str().unwrap_or("");
    let logo = config["logo"].as_str().unwrap_or("");
    let loading_bar_color = config["loading_bar_color"].as_str().unwrap_or("#FFD740");
    let loading_bar_background = config["loading_bar_background"].as_str().unwrap_or("#222222");

    // Show splash screen (placeholder)
    println!("[SPLASH] Showing splash screen: {}", splash_screen);
    println!("[LOGO] Showing logo: {}", logo);
    println!("[LOADING] Bar color: {}, background: {}", loading_bar_color, loading_bar_background);

    // Simulate loading bar
    for i in 0..=100 {
        print!("\r[LOADING] {}%", i);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(10));
    }
    println!("\n[LOADING] Complete!");

    if enable_installer && !dev_mode {
        // Run installer (placeholder)
        println!("[INSTALLER] Running installer...");
        // Here you would launch the installer UI or process
        // For now, just simulate
        thread::sleep(Duration::from_secs(2));
        println!("[INSTALLER] Install complete!");
    } else {
        println!("[LAUNCHER] Installer disabled (dev mode or config)");
    }

    // Launch main engine
    let exe_path = "enchantmen-creator.exe";
    if Path::new(exe_path).exists() {
        println!("[LAUNCHER] Launching {}...", exe_path);
        // On Windows, use std::process::Command
        let _ = std::process::Command::new(exe_path).spawn();
    } else {
        println!("[ERROR] Engine executable not found: {}", exe_path);
    }
}
