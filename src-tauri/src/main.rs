#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_video_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_video_files(path: String) -> Vec<String> {
    let mut files = Vec::new();

    for file in WalkDir::new(path || "C:/Users/Elito/Videos/Videos").into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            if file.path().extension() == Some("mp4".as_ref()) || file.path().extension() == Some("mkv".as_ref()) {
                files.push(file.path().to_str().unwrap().to_string());
            }
        }
    }

    files.into()
}

