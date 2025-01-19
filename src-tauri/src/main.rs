// multi-level
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            print!("Hello from setup");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
