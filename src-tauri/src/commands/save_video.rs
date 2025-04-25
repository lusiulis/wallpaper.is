use tauri::AppHandle;

use crate::fs_utils::get_videos_dir;

#[tauri::command]
pub fn save_video_from_buffer(
    app: AppHandle,
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
