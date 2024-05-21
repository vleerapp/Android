
// mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Current OS: {}", std::env::consts::OS);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commands::download,
            // commands::get_music_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}