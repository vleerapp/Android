// use anyhow::anyhow;
// use rusty_ytdl::Video;
// use std::path::PathBuf;
// use std::fs;
// use tauri::Result as TauriResult;

// #[tauri::command]
// pub async fn download(url: String, name: String) -> TauriResult<()> {
//     let video = Video::new(url.clone()).map_err(|e| anyhow!(e.to_string()))?;

//     let base_path = get_music_path();

//     let mut path = base_path.clone();
//     path.push("Songs");
//     path.push(&name);

//     video
//         .download(&path)
//         .await
//         .map_err(|e| anyhow!(e.to_string()))?;

//     println!("Downloaded: {}", path.display());
//     Ok(())
// }

// #[tauri::command]
// pub fn get_music_path() -> PathBuf {
//     let mut path = PathBuf::new();
//     match std::env::consts::OS {
//         "macos" => {
//             let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
//             path.push(format!("/users/{}/Music/Vleer", username));
//         }
//         "linux" => {
//             let username = std::env::var("USER").unwrap_or_else(|_| "default".into());
//             path.push(format!("/home/{}/Music/Vleer", username));
//         }
//         "windows" => {
//             let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
//             path.push(format!("C:\\Users\\{}\\Music\\Vleer", username));
//         }
//         _ => {}
//     }
//     if !path.exists() {
//         fs::create_dir_all(&path).expect("Failed to create music directory");
//     }
//     path.push("Songs");
//     if !path.exists() {
//         fs::create_dir_all(&path).expect("Failed to create Songs directory");
//     }
//     path.pop();

//     path.push("Covers");
//     if !path.exists() {
//         fs::create_dir_all(&path).expect("Failed to create Covers directory");
//     }
//     path.pop();
//     return path;
// }