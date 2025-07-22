import styles from './itemList.module.scss';

const ItemList = ({ items }: { items: TreeNode[] }) => {
  const handleChangeIndentifierColor = (id: string) => {
    console.log('change color id: ', id);
  };

  const handleItemClick = (item: TreeNode) => {
    console.log('change color item: ', item);
  };

  const ItemNode = (item: TreeNode) => (
    <div
      className={'children' in item ? styles.folder : styles.item}
      key={item.id}
    >
      <div className={styles.node}>
        <p
          className={styles.indentifier}
          onClick={() => handleChangeIndentifierColor(item.id)}
        />
        <p
          className={styles.label}
          onClick={() => handleItemClick(item)}
        >
          {item.value}
        </p>
      </div>
      {'children' in item && item.children.length > 0 && (
        <ItemList items={item.children} />
      )}
    </div>
  );

  return (
    <div className={styles.container}>
      {items.map((item) => ItemNode(item))}
    </div>
  );
};

export default ItemList;
