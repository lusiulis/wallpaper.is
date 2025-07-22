// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod mpv_controller;
mod system_tray;
mod windows_utils;

use commands::set_wallpaper::set_video_as_wallpaper;
use commands::get_items::get_items;
use commands::add_folder::add_folder;
use db::init_db;
use system_tray::create_tray_icon;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            init_db(&app.handle())?;
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
        .invoke_handler(tauri::generate_handler![set_video_as_wallpaper, get_items, add_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
