import { useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import styles from './addWallpaper.module.scss';

const MAX_VIDEO_SIZE = 10 * 1024 * 1024; // 10MB

const AddWallpaper = () => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const hadleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    if (file.type !== 'video/mp4') {
      //alert
      fileInputRef.current!.value = '';
      return;
    }

    if (file.size > MAX_VIDEO_SIZE) {
      //aler
      fileInputRef.current!.value = ''; // Limpiar el input
      return;
    }

    //.....

    const selectedFile = file;

    // Leer el archivo como un array de bytes
    const reader = new FileReader();
    
    console.log('hello? 1');
    reader.onload = async (event) => {
      if (event.target?.result) {
        const buffer = new Uint8Array(event.target.result as ArrayBuffer); // El buffer del archivo
        const fileName = selectedFile.name;

        try {
          console.log('hello?');
          // Llamar al comando Tauri para guardar el archivo
          const videoPath = await invoke('save_video_from_buffer', {
            buffer: Array.from(buffer), // Convertir el buffer a un array de números
            name: fileName,
          });

          console.log(`Archivo guardado en: ${videoPath}`);
          invoke('set_video_as_wallpaper', { videoPath });
        } catch (error) {
          console.error('Error al guardar el archivo:', error);
        }
      }
    };

    reader.readAsArrayBuffer(selectedFile); // Leer el archivo como ArrayBuffer

    /* const video_path = URL.createObjectURL(file);
    //.....
     */
  };

  return (
    <div className={styles.container}>
      <div className={styles.inputContainer}>
        <p className={styles.label}>Add your own live wallpapers!</p>
        <label
          htmlFor='file-upload'
          className={styles.customLabel}
        >
          Add
        </label>
        <input
          accept='video/mp4'
          ref={fileInputRef}
          id='file-upload'
          type='file'
          className={styles.input}
          onChange={hadleFileChange}
        />
      </div>
      <div className={styles.fun}>
        <div className={`${styles.block} ${styles.first}`} />
        <div className={`${styles.block} ${styles.second}`} />
        <div className={`${styles.block} ${styles.third}`} />
      </div>
    </div>
  );
};

export default AddWallpaper;
