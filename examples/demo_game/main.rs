//! Demo game using Enchentment Engine core systems

use engine::keybinds::Keybinds;
use engine::macros::MacroSystem;

fn main() {
    // Setup keybinds for RPG controls
    let mut kb = Keybinds::new();
    kb.bind("MoveUp", "W");
    kb.bind("MoveDown", "S");
    kb.bind("MoveLeft", "A");
    kb.bind("MoveRight", "D");
    kb.bind("Attack", "Space");
    kb.bind("Inventory", "I");
    println!("[DEMO] Keybinds: MoveUp = {:?}, Attack = {:?}", kb.get_key("MoveUp"), kb.get_key("Attack"));

    // Setup a macro for quick save
    let mut ms = MacroSystem::new();
    ms.add_macro("QuickSave", vec!["Save".to_string(), "Notify".to_string()]);
    println!("[DEMO] Running QuickSave macro:");
    ms.run_macro("QuickSave");

    // Simulate a game loop
    for frame in 0..3 {
        println!("[DEMO] Frame {}", frame);
        // Simulate input and macro usage
        if frame == 1 {
            println!("[DEMO] Player pressed F5 (QuickSave)");
            ms.run_macro("QuickSave");
        }
    }
    println!("[DEMO] Demo game finished.");
}
