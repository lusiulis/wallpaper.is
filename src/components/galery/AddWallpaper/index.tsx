import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import styles from './addWallpaper.module.scss';


const AddWallpaper = () => {
  const selectFile = async () => {
    const videoPath = await open({
      multiple: false,
      filters: [{ name: 'Videos', extensions: ['mp4'] }],
    });

    if(videoPath) {
      invoke('set_video_as_wallpaper', { videoPath });
    }
  }

  return (
    <div className={styles.container}>
      <div className={styles.inputContainer}>
        <p className={styles.label}>Add your own live wallpapers!</p>
        
        <button onClick={selectFile} className={styles.customLabel}>
          Add
        </button>
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
