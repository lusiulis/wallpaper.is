use once_cell::sync::Lazy;
use serde_json::json;
use std::path::PathBuf;
use std::process::Command;
use std::{fs::OpenOptions, io::Write, sync::Mutex};

const MPV_SOCKET: &str = r"\\.\pipe\mpv-pipe";

pub static MPV_CONTROLLER: Lazy<Mutex<Option<MpvControl>>> = Lazy::new(|| Mutex::new(None));

pub struct MpvControl {
    pub child: Option<std::process::Child>,
}

fn send_mpv_command(json: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(false)
        .open(MPV_SOCKET)
        .map_err(|e| format!("No se pudo abrir el socket: {}", e))?;
    file.write_all(format!("{}\n", json).as_bytes())
        .map_err(|e| format!("Error al escribir en el socket: {}", e))?;
    Ok(())
}

pub fn play_video(video_path: &str, mpv_path: PathBuf) -> Result<String, String> {
    let mut mpv_lock = MPV_CONTROLLER.lock().unwrap();

    if let Some(mpv_control) = mpv_lock.as_mut() {
        if let Some(child) = &mut mpv_control.child {
            match child.try_wait() {
                Ok(Some(_status)) => {
                    // Murió, reiniciamos
                    mpv_control.child = None;
                }
                Ok(None) => {
                    // Sigue vivo, solo reemplazamos el video
                    replace(video_path)?;
                    return Ok("replaced".to_string());
                }
                Err(err) => {
                    return Err(format!(
                        "Error al verificar el estado del proceso MPV: {}",
                        err
                    ));
                }
            }
        }
    }

    let _child = Command::new(mpv_path)
        .args([
            &video_path,
            "--input-ipc-server=\\\\.\\pipe\\mpv-pipe",
            "--loop",
            "--idle=yes",
            "--force-window=yes",
            "--no-border",
            "--fs",
            "--no-keepaspect",
            "--geometry=100%x100%",
            "--osc=no",
            "--no-input-default-bindings",
            "--quiet",
            "--no-audio",
        ])
        .spawn()
        .map_err(|e| format!("Error al iniciar MPV: {}", e))?;

    *mpv_lock = Some(MpvControl {
        child: Some(_child),
    });

    drop(mpv_lock);
    Ok("started".to_string())
}

pub fn pause() -> Result<(), String> {
    match send_mpv_command(r#"{"command": ["set_property", "pause", true]}"#) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error al enviar comando: {}", e)),
    }
}

pub fn play() -> Result<(), String> {
    match send_mpv_command(r#"{"command": ["set_property", "pause", false]}"#) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error al enviar comando: {}", e)),
    }
}

pub fn quit() -> Result<(), String> {
    match send_mpv_command(r#"{"command": ["quit"]}"#) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error al enviar comando: {}", e)),
    }
}

pub fn replace(path: &str) -> Result<(), String> {
    let command = json!({
        "command": ["loadfile", path, "replace"]
    });
    match send_mpv_command(&command.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error al enviar comando: {}", e)),
    }
}
