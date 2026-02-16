#![windows_subsystem = "windows"]

use rustapi::editor::Editor;
use theframework::*;

fn main() {
    println!("[DEBUG] main() started");
    let args: Vec<_> = std::env::args().collect();

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = rustapi::i18n::select_system_locales();

    let editor = Editor::new();
    println!("[DEBUG] Editor constructed");
    let mut app = TheApp::new();
    println!("[DEBUG] TheApp constructed");
    app.set_cmd_line_args(args);

    println!("[DEBUG] Running TheApp");
    let () = app.run(Box::new(editor));
    println!("[DEBUG] TheApp exited");
}
