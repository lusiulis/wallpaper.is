import GaleryMenu from '@components/galery/menu';
import Logo from '@assets/images/Logo_naked.svg';
import Button from '@components/common/button';
import AddWallpaper from '@components/galery/AddWallpaper';
import styles from './galery.module.scss';

const Galery = () => {
  const handleButtonClick = () => {

  };

  return (
    <div className={styles.overall}>
      <GaleryMenu />
      <div className={styles.content}>
        <div className={styles.hero}>
          <img
            src={Logo}
            alt='Logo'
            className={styles.logo}
          />
          <div className={styles.text}>
            <h1>Wallpaper-is..!</h1>
            <p>
              Wallpaper is a website that allows you to download wallpapers from
              the internet. Make sure to check out the different categories and
              search for your favorite wallpaper. Have fun and enjoy your
              browsing!
            </p>
          </div>
        </div>

        <div className={styles.actions}>
          <AddWallpaper />
          <p>
            Wallpaper is a website that allows you to download wallpapers from
            the internet. Make sure to check out the different categories and
            search for your favorite wallpaper. Have fun and enjoy your
            browsing!
          </p>
          <Button
            onClick={handleButtonClick}
            style='Green'
          >
            Look For Online Wallpapers
          </Button>
        </div>
      </div>
    </div>
  );
};

export default Galery;
