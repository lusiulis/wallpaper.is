import styles from './button.module.scss'

type IButtonProps = {
  children: React.ReactNode;
  onClick?: () => void;
  style: 'Green' | 'Purple' | 'Orange' | 'White' | 'Red';
};

const Button = ({ children, style, onClick }: IButtonProps) => {
  return (
    <button
      onClick={onClick}
      className={`${styles.button} ${styles[style]}`}
    >
      {children}
    </button>
  );
};

export default Button;
