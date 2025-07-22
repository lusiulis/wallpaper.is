import { useItemContext } from '@app/context/ItemsContex';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useEffect, useMemo, useState } from 'react';

const useItemAPI = () => {
  const { dispatch, state } = useItemContext();

  const fetchItems = async () => {
    const items = (await invoke('get_items')) as TreeNode[];
    dispatch({
      type: 'SET_ITEMS',
      payload: items,
    });
  };

  useEffect(() => {
    fetchItems();
  }, []);

  const addAPIFolder = async (value: string) => {
    try {
      const result = await invoke('add_folder', {
        value,
        parent: null,
      });
      console.log('Added folder: ', result);

      dispatch({
        type: 'ADD_ITEM',
        payload: result as Folder,
      });
    } catch (e) {
      console.log("Couldn't add the folder: ", e);
    }
  };

  const removeAPIItem = async (id: string) => {
    try {
      await invoke('remove_item', {
        id,
      });
      dispatch({
        type: 'REMOVE_ITEM',
        payload: { id },
      });
    } catch (e) {
      console.log("Couldn't remove the item: ", e);
    }
  };

  const updateAPIItem = async (id: string, value: string, parent: string) => {
    try {
      await invoke('update_item', { id, value, parent });
      await fetchItems();
    } catch (e) {
      console.error("Couldn't update the item:", e);
    }
  };

  const addVideo = async () => {
    try {
      const videoPath = await open({
        multiple: false,
        filters: [{ name: 'Videos', extensions: ['mp4'] }],
      });

      if (videoPath) {
        await invoke('set_video_as_wallpaper', { videoPath });

        fetchItems();
      }
    } catch (e) {
      console.error("Couldn't add the video:", e);
    }
  };

  const [searchQuery, setSearchQuery] = useState('');

  const filteredItems = useMemo(() => {
    const lowerQuery = searchQuery.toLowerCase();
    return state.items.filter((item) =>
      item.value.toLowerCase().includes(lowerQuery)
    );
  }, [searchQuery, state.items]);

  const searchStateItem = async (value: string) => {
    setSearchQuery(value);
  };

  return {
    addAPIFolder,
    removeAPIItem,
    updateAPIItem,
    searchStateItem,
    filteredItems,
    addVideo,
  };
};
export default useItemAPI;
