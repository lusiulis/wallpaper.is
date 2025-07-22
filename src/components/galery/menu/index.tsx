import styles from './menu.module.scss';

const GaleryMenu = () => {
  /*const [showAddFolder, setShowAddFolder] = useState<boolean>(false);
  const [searchItem, setSearchItem] = useState<string>('');
  const [newFolder, setNewFolder] = useState<string>('');

  const { searchStateItem, filteredItems, addAPIFolder } = useItemAPI();

  const showNewFolder = () => {
    setShowAddFolder(!showAddFolder);
  };

  const handleNewFolderInputChange = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    setNewFolder(e.target.value);
  };

  const handleSearchInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchItem(e.target.value);
  };

  const addNewFolder = async () => {
    await addAPIFolder(newFolder);
    setNewFolder('');
    setShowAddFolder(!showAddFolder);
  };

  const searchItems = () => {
    searchStateItem(searchItem);
  };*/

  return (
    <div className={styles.menu}>
      <div className={styles.container}>
        TODO
        {/*
          
          <h3 className={styles.title}>Recently Played</h3>
        {
          //ADD RECENT VIDEOS LIST
        }
      </div>
      <div className={styles.container}>
        <div className={styles.header}>
          <h3 className={styles.title}>Galery</h3>
          <img
            src={addLogo}
            alt='Add new item'
            className={styles.addNewItem}
            onClick={showNewFolder}
          />
        </div>

        <div className={styles.actionContainer}>
          <input
            type='text'
            placeholder='Search...'
            onChange={handleSearchInputChange}
            className={styles.input}
          />
          <button
            className={styles.btn}
            onClick={searchItems}
          >
            Search
          </button>
        </div>

        {showAddFolder && (
          <div className={styles.actionContainer}>
            <button
              className={styles.btn}
              onClick={addNewFolder}
            >
              Add
            </button>
            <input
              type='text'
              value={newFolder}
              onChange={handleNewFolderInputChange}
              placeholder='New Folder...'
              className={styles.input}
            />
          </div>
        )}
        <ItemList items={filteredItems} />
          */}
      </div>
    </div>
  );
};

export default GaleryMenu;
