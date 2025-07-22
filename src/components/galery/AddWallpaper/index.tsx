import useItemAPI from '@app/hooks/useItemApi';
import styles from './addWallpaper.module.scss';

const AddWallpaper = () => {
  const { addVideo } = useItemAPI();

  return (
    <div className={styles.container}>
      <div className={styles.inputContainer}>
        <p className={styles.label}>Add your own live wallpapers!</p>

        <button
          onClick={addVideo}
          className={styles.customLabel}
        >
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
