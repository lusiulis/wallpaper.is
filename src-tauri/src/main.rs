// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod fs_utils;
mod windows_utils;

use commands::{save_video::save_video_from_buffer, set_wallpaper::set_video_as_wallpaper};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_video_from_buffer,
            set_video_as_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
