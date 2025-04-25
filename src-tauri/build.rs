use std::fs::{create_dir_all, File};
use std::path::Path;

fn main() {
    let mpv_path = Path::new("resources/mpv/mpv.exe");

    // 🔧 Crear carpeta si no existe
    if let Some(parent) = mpv_path.parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("No se pudo crear la carpeta resources");
        }
    }
    if !mpv_path.exists() {
        println!("cargo:warning=mpv.exe no encontrado, descargando...");

        let url = "https://github.com/lusiulis/wallpaper.is/releases/download/mpv/mpv.exe";

        let mut response = reqwest::blocking::get(url).expect("Fallo al descargar mpv.exe");
        let mut out_file = File::create(&mpv_path).expect("No se pudo crear mpv.exe");

        response
            .copy_to(&mut out_file)
            .expect("No se pudo guardar mpv.exe");

        println!("cargo:warning=mpv.exe descargado correctamente.");
    } else {
        println!("cargo:warning=mpv.exe ya existe, se omite la descarga.");
    }

    tauri_build::build()
}
