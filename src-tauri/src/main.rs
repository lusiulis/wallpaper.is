// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod mpv_controller;
mod system_tray;
mod windows_utils;

use commands::set_wallpaper::set_video_as_wallpaper;
use system_tray::create_tray_icon;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            create_tray_icon(app);
            Ok(())
        })
        .on_window_event(|app, event| match event {
            tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed => {
                let _ = crate::mpv_controller::quit();
                app.close().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![set_video_as_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
