#![windows_subsystem = "windows"]

use rustapi::editor::Editor;
use theframework::*;

use std::panic;
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
    let result = panic::catch_unwind(|| {
        app.run(Box::new(editor));
    });
    match result {
        Ok(_) => println!("[DEBUG] TheApp exited normally"),
        Err(e) => {
            println!("[ERROR] TheApp panicked!");
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("Panic info: {}", s);
            } else if let Some(s) = e.downcast_ref::<String>() {
                println!("Panic info: {}", s);
            } else {
                println!("Panic occurred but couldn't get info.");
            }
        }
    }
}
