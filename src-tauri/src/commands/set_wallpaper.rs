use tauri::{path::BaseDirectory, AppHandle, Manager};
use windows::core::s;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{EnumWindows, FindWindowA, FindWindowExA, SendMessageA, SetParent},
};

use crate::mpv_controller::play_video;
use crate::windows_utils::{enum_window, wait_for_mpv_window};

#[tauri::command]
pub fn set_video_as_wallpaper(app: AppHandle, video_path: String) -> Result<(), String> {
    let mpv_path = app
        .path()
        .resolve("resources/mpv/mpv.exe", BaseDirectory::Resource)
        .map_err(|e| format!("No se pudo obtener la ruta de MPV: {:?}", e))?;

    if !mpv_path.exists() {
        return Err("mpv.exe no fue encontrado en resources/mpv".to_string());
    }

    let video_result: Result<String, String> = play_video(&video_path, mpv_path);

    if video_result.is_err() {
        return Err("No se pudo reproducir el video".to_string());
    }

    let unwraped_video_result = video_result.unwrap();

    match unwraped_video_result.as_str() {
        "started" => {
            let mpv_window: HWND = wait_for_mpv_window()
                .ok_or("No se encontró la ventana de mpv dentro del tiempo límite")?;

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
                        FindWindowExA(Some(progman), Some(HWND::default()), s!("WorkerW"), None)
                            .unwrap();
                }

                if HWND::is_invalid(&worker_w) {
                    return Err("No se pudo encontrar una ventana WorkerW válida".to_string());
                }

                SetParent(mpv_window, Some(worker_w))
                    .map(|_| ())
                    .map_err(|e| {
                        format!("No se pudo establecer la ventana de MPV como fondo: {}", e)
                    })
            }
        }
        "replaced" => Ok(()),
        _ => {
            return Err("No se pudo reproducir el video".to_string());
        }
    }
}
