use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle,
};

use crate::mpv_controller::{pause, play, quit};

fn quit_action(app: &AppHandle) {
    let _ = quit();
    app.exit(0);
}

fn pause_action() {
    let _ = pause();
}

fn play_action() {
    let _ = play();
}

pub fn create_tray_icon(app: &mut tauri::App) {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let pause_i = MenuItem::with_id(app, "pause", "Pause", true, None::<&str>).unwrap();
    let play_i = MenuItem::with_id(app, "play", "Play", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i, &pause_i, &play_i]);
    if menu.is_ok() {
        let _tray = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu.unwrap())
            .on_menu_event(|app_handle, event| match event.id.as_ref() {
                "quit" => quit_action(app_handle),
                "pause" => pause_action(),
                "play" => play_action(),
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }
            })
            .build(app);
    }
}
