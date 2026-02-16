use launcher_api::{LauncherConfig, show_splash, show_logo, show_loading_bar, run_installer, launch_engine};

fn main() {
    let config = LauncherConfig::load("../launcher_config.toml");
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, "Loading Engine...");
    if config.enable_installer && !config.dev_mode {
        run_installer(&config);
    }
    launch_engine(&config);
}
