use std::os::windows::ffi::OsStrExt;
use std::{thread::sleep, time::Duration};
use windows::core::{s, BOOL, PCWSTR};
use windows::Win32::{
    Foundation::{HWND, LPARAM},
    UI::WindowsAndMessaging::*,
};

pub fn wait_for_mpv_window() -> Option<HWND> {
    let mut mpv_window: Option<HWND> = None;

    while mpv_window.is_none() {
        mpv_window = find_window_by_class("mpv");
        sleep(Duration::from_millis(100)); // Esperar brevemente antes de comprobar nuevamente
    }
    mpv_window
}

pub fn find_window_by_class(class_name: &str) -> Option<HWND> {
    let wide: Vec<u16> = std::ffi::OsString::from(class_name)
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0)) // null terminator
        .collect();

    let result = unsafe { FindWindowW(PCWSTR(wide.as_ptr()), PCWSTR::null()) };
    match result {
        Ok(hwnd) if !hwnd.is_invalid() => Some(hwnd),
        _ => None,
    }
}

pub extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    unsafe {
        let shell_dll_def_view = FindWindowExA(
            Some(window),
            Some(HWND::default()),
            s!("SHELLDLL_DefView"),
            None,
        )
        .unwrap_or(HWND::default());

        if !HWND::is_invalid(&shell_dll_def_view) {
            let worker_w = FindWindowExA(Some(HWND::default()), Some(window), s!("WorkerW"), None)
                .unwrap_or(HWND::default());

            if !HWND::is_invalid(&worker_w) {
                *(ref_worker_w.0 as *mut HWND) = worker_w;
            }
        }

        BOOL(1)
    }
}
