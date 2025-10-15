#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
use std::fs;

#[command]
fn fetch_game_list() -> Result<Vec<String>, String> {
    // Placeholder: Replace with actual API request (reqwest to your webserver)
    Ok(vec![
        "Shadow of the Beast".to_string(),
        "Lotus Turbo Challenge 2".to_string(),
        "Sensible Soccer".to_string(),
    ])
}

#[command]
fn download_game(id: i32) -> Result<String, String> {
    // TODO: Implement real HTTP download logic with reqwest
    Ok(format!("Downloading game with ID {}", id))
}

#[command]
fn upload_game(file_path: String) -> Result<String, String> {
    // TODO: Upload logic (multipart POST)
    if fs::metadata(&file_path).is_ok() {
        Ok(format!("Uploading file: {}", file_path))
    } else {
        Err("File not found".into())
    }
}

#[command]
fn launch_fsuae(config_path: String) -> Result<String, String> {
    let exe_path = if cfg!(target_os = "windows") {
        "src-tauri/binaries/windows/fs-uae.exe"
    } else if cfg!(target_os = "macos") {
        "src-tauri/binaries/macos/fs-uae"
    } else {
        "src-tauri/binaries/linux/fs-uae"
    };

    Command::new(exe_path)
        .arg(config_path)
        .spawn()
        .map_err(|e| format!("Failed to launch FS-UAE: {}", e))?;

    Ok("FS-UAE launched".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_game_list,
            download_game,
            upload_game,
            launch_fsuae
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
