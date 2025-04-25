use std::os::windows::ffi::OsStrExt;
use windows::Win32::{
    Foundation::{ HWND, LPARAM}, 
    UI::WindowsAndMessaging::*,
};
use windows::core::{s, BOOL, PCWSTR};

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
