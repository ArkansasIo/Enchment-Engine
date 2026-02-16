//! Full-featured installer app for Enchantment Engine

use installer_api::{InstallerConfig, show_splash, show_logo, show_loading_bar, perform_install};

fn main() {
    let config = InstallerConfig::load("../launcher_config.toml");
    show_splash(&config);
    show_logo(&config);
    show_loading_bar(&config, "Installing...");
    perform_install(&config);
}
