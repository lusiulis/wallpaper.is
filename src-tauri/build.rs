use std::fs::{create_dir_all, File};
use std::path::Path;

fn main() {
    let mpv_path = Path::new("resources/mpv/mpv.exe");

    // 🔧 Crear carpeta si no existe
    if let Some(parent) = mpv_path.parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("The folder creation failed.");
        }
    }
    if !mpv_path.exists() {
        println!("cargo:warning=mpv.exe NOT FOUND, downloading...");

        let url = "https://github.com/lusiulis/wallpaper.is/releases/download/mpv/mpv.exe";

        let mut response = reqwest::blocking::get(url).expect("Failed to download mpv.exe");
        let mut out_file = File::create(&mpv_path).expect("mpv.exe creation failed.");

        response
            .copy_to(&mut out_file)
            .expect("mpv.exe copy failed.");

        println!("cargo:warning=mpv.exe downloaded successfully.");
    } else {
        println!("cargo:warning=mpv.exe already exists.");
    }

    tauri_build::build()
}
