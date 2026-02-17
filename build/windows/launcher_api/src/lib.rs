use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use chrono::Local;

pub fn log_error(msg: &str) {
    let now = Local::now();
    let entry = format!("[{}] ERROR: {}\n", now.format("%Y-%m-%d %H:%M:%S"), msg);
    let mut file = OpenOptions::new().create(true).append(true).open("launcher_error.log").unwrap();
    let _ = file.write_all(entry.as_bytes());
}

pub fn log_info(msg: &str) {
    let now = Local::now();
    let entry = format!("[{}] INFO: {}\n", now.format("%Y-%m-%d %H:%M:%S"), msg);
    let mut file = OpenOptions::new().create(true).append(true).open("launcher_error.log").unwrap();
    let _ = file.write_all(entry.as_bytes());
}

pub fn show_version(config: &LauncherConfig) {
    println!("[VERSION] Launcher for Enchentment Engine v0.8.100");
    println!("[DEVELOPER] Markus Moenig");
    println!("[ENGINE EXE] {}", config.engine_exe);
}

pub fn launch_engine_with_args(config: &LauncherConfig, args: &[&str]) {
    println!("[ENGINE] Launching {} with args {:?}...", config.engine_exe, args);
    match Command::new(&config.engine_exe).args(args).spawn() {
        Ok(_) => log_info("Engine launched successfully."),
        Err(e) => log_error(&format!("Failed to launch engine: {}", e)),
    }
}

/// Loads config, shows splash/logo/loading, runs installer if needed, launches engine, and logs errors.
pub fn run_full_launcher_flow(config_path: &str, args: &[&str]) {
    let config = LauncherConfig::load(config_path);
    show_version(&config);
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, "Loading Engine...");
    if config.enable_installer && !config.dev_mode {
        run_installer(&config);
    }
    launch_engine_with_args(&config, args);
}
use std::fs;
use toml::Value;

pub struct LauncherConfig {
    pub enable_installer: bool,
    pub dev_mode: bool,
    pub splash_screen: String,
    pub logo: String,
    pub loading_bar_color: String,
    pub loading_bar_background: String,
    pub engine_exe: String,
}

impl LauncherConfig {
    pub fn load(path: &str) -> Self {
        let config = fs::read_to_string(path).expect("Failed to read config");
        let value: Value = toml::from_str(&config).expect("Invalid TOML");
        Self {
            enable_installer: value["enable_installer"].as_bool().unwrap_or(true),
            dev_mode: value["dev_mode"].as_bool().unwrap_or(false),
            splash_screen: value["splash_screen"].as_str().unwrap_or("").to_string(),
            logo: value["logo"].as_str().unwrap_or("").to_string(),
            loading_bar_color: value["loading_bar_color"].as_str().unwrap_or("#FFD740").to_string(),
            loading_bar_background: value["loading_bar_background"].as_str().unwrap_or("#222222").to_string(),
            engine_exe: value["engine_exe"].as_str().unwrap_or("../enchantmen-creator.exe").to_string(),
        }
    }
}

pub fn show_splash(config: &LauncherConfig) {
    println!("[SPLASH] {}", config.splash_screen);
}

pub fn show_logo(config: &LauncherConfig) {
    println!("[LOGO] {}", config.logo);
}

pub fn show_loading_bar(config: &LauncherConfig, msg: &str) {
    println!("[LOADING] {} (bar: {}, bg: {})", msg, config.loading_bar_color, config.loading_bar_background);
    for i in 0..=100 {
        print!("\r[LOADING] {}%", i);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(8));
    }
    println!("\n[LOADING] Complete!");
}

pub fn run_installer(config: &LauncherConfig) {
    println!("[INSTALLER] Launching installer...");
    let _ = std::process::Command::new("../installer/installer.exe").spawn();
}

pub fn launch_engine(config: &LauncherConfig) {
    println!("[ENGINE] Launching {}...", config.engine_exe);
    let _ = std::process::Command::new(&config.engine_exe).spawn();
}
