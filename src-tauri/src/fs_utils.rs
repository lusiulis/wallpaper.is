use tauri::{AppHandle, Manager};

pub fn get_videos_dir(app: AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Error obteniendo AppData: {:?}", e))?
        .join("wallpaper.is")
        .join("videos");

    Ok(path.to_string_lossy().to_string())
}
