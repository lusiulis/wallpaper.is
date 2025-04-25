use std::os::windows::ffi::OsStrExt;
use std::process::Command;
use std::time::Duration;
use tauri::path::BaseDirectory;
use tauri::Manager;
use windows::core::{s, BOOL, PCWSTR};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, FindWindowW, SendMessageA, SetParent,
};

fn get_videos_dir(app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Error obteniendo AppData: {:?}", e))?
        .join("wallpaper.is")
        .join("videos");

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn save_video_from_buffer(
    app: tauri::AppHandle,
    buffer: Vec<u8>,
    name: String,
) -> Result<String, String> {
    use std::fs;
    let dir_str = get_videos_dir(app)?;
    let dir = std::path::Path::new(&dir_str);
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Error creando carpeta: {}", e))?;
    }

    let file_path = dir.join(name); // ej: "video123.mp4"
    fs::write(&file_path, buffer).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

fn find_window_by_class(class_name: &str) -> Option<HWND> {
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

extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
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

#[tauri::command]
fn set_video_as_wallpaper(app: tauri::AppHandle, video_path: String) -> Result<(), String> {
    // Encuentra el manejador de la ventana del escritorio en Windows
    let mpv_path = app
        .path()
        .resolve("resources/mpv/mpv.exe", BaseDirectory::Resource)
        .map_err(|e| format!("No se pudo obtener la ruta de MPV: {:?}", e))?;

    println!("Ruta resuelta de MPV: {:?}", mpv_path);

    if !mpv_path.exists() {
        return Err("mpv.exe no fue encontrado en resources/mpv".to_string());
    }

    // Ejecutar el reproductor MPV en segundo plano con el video
    let _child = Command::new(mpv_path)
        .args([
            &video_path,
            "--loop",                      // Repite el video
            "--no-border",                 // Elimina bordes y barra de título
            "--geometry=100%x100%",        // Ocupa toda la pantalla
            "--osc=no",                    // No muestra controles de reproducción
            "--no-input-default-bindings", // Desactiva teclas por defecto
            "--quiet",                     // No muestra mensajes en consola
            "--no-audio",                  // Desactiva sonido
        ])
        .spawn()
        .map_err(|e| format!("Error al iniciar MPV: {}", e))?;

    std::thread::sleep(Duration::from_secs(2)); // Ajusta este tiempo si es necesario

    // Obtener la ventana de MPV
    let mpv_window: HWND = find_window_by_class("mpv").ok_or("No se encontró la ventana de mpv")?;

    unsafe {
        let progman = FindWindowA(s!("Progman"), None).unwrap();

        // Envía el mensaje mágico a Progman para crear un WorkerW limpio
        SendMessageA(progman, 0x052C, WPARAM(0), LPARAM(0));

        // Espera un poco para que el sistema tenga tiempo de procesar

        // Ahora intentas encontrar el WorkerW como siempre
        let mut worker_w = HWND::default();

        EnumWindows(
            Some(enum_window),
            LPARAM(&mut worker_w as *mut HWND as isize),
        )
        .unwrap();

        if HWND::is_invalid(&worker_w) {
            // Fallback: intentar encontrarlo directamente desde Progman
            worker_w =
                FindWindowExA(Some(progman), Some(HWND::default()), s!("WorkerW"), None).unwrap();
        }

        if HWND::is_invalid(&worker_w) {
            return Err("No se pudo encontrar una ventana WorkerW válida".to_string());
        }

        // Finalmente, mover la ventana mpv como hija de WorkerW
        let result = SetParent(mpv_window, Some(worker_w));
        match result {
            Ok(_) => Ok(()), // Si la operación fue exitosa
            Err(e) => Err(format!(
                "No se pudo establecer la ventana de MPV como fondo: {}",
                e
            )), // Si hubo un error
        }
    }
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_video_as_wallpaper,
            save_video_from_buffer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
