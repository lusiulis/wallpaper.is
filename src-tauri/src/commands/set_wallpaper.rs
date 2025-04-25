use std::{process::Command, thread::sleep, time::Duration};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use windows::core::s;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::*,
};

use crate::windows_utils::{enum_window, find_window_by_class};

#[tauri::command]
pub fn set_video_as_wallpaper(app: AppHandle, video_path: String) -> Result<(), String> {
    let mpv_path = app
        .path()
        .resolve("resources/mpv/mpv.exe", BaseDirectory::Resource)
        .map_err(|e| format!("No se pudo obtener la ruta de MPV: {:?}", e))?;

    if !mpv_path.exists() {
        return Err("mpv.exe no fue encontrado en resources/mpv".to_string());
    }

    let _child = Command::new(mpv_path)
        .args([
            &video_path,
            "--loop",
            "--no-border",
            "--geometry=100%x100%",
            "--osc=no",
            "--no-input-default-bindings",
            "--quiet",
            "--no-audio",
        ])
        .spawn()
        .map_err(|e| format!("Error al iniciar MPV: {}", e))?;

    sleep(Duration::from_secs(2));

    let mpv_window: HWND = find_window_by_class("mpv").ok_or("No se encontró la ventana de mpv")?;

    unsafe {
        let progman = FindWindowA(s!("Progman"), None).unwrap();
        SendMessageA(progman, 0x052C, WPARAM(0), LPARAM(0));

        let mut worker_w = HWND::default();
        EnumWindows(
            Some(enum_window),
            LPARAM(&mut worker_w as *mut HWND as isize),
        )
        .unwrap();

        if HWND::is_invalid(&worker_w) {
            worker_w =
                FindWindowExA(Some(progman), Some(HWND::default()), s!("WorkerW"), None).unwrap();
        }

        if HWND::is_invalid(&worker_w) {
            return Err("No se pudo encontrar una ventana WorkerW válida".to_string());
        }

        SetParent(mpv_window, Some(worker_w))
            .map(|_| ())
            .map_err(|e| format!("No se pudo establecer la ventana de MPV como fondo: {}", e))
    }
}
