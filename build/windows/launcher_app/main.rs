//! Full-featured launcher app for Enchantment Engine

use launcher_api::{run_full_launcher_flow};

fn main() {
    // Collect command-line arguments (excluding program name)
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    // Run the full launcher flow with logging, version info, and custom args
    run_full_launcher_flow("../launcher_config.toml", &arg_refs);
}
