import { createContext, useContext, useReducer } from 'react';

type Action =
  | { type: 'ADD_ITEM'; payload: TreeNode }
  | { type: 'REMOVE_ITEM'; payload: { id: string } }
  | { type: 'SET_ITEMS'; payload: TreeNode[] };

type ItemsState = {
  items: TreeNode[];
  recentVideos: Item[];
};

type ItemContextType = {
  state: ItemsState;
  dispatch: React.Dispatch<Action>;
};

const itemsReducer = (state: ItemsState, action: Action): ItemsState => {
  switch (action.type) {
    case 'ADD_ITEM':
      return {
        ...state,
        items: [...state.items, action.payload],
      };
    case 'REMOVE_ITEM':
      return {
        ...state,
        items: state.items.filter((item) => item.id !== action.payload.id),
      };
    case 'SET_ITEMS':
      return {
        ...state,
        items: action.payload,
      };
    default:
      return state;
  }
};

const ItemContext = createContext<ItemContextType | undefined>(undefined);

export const ItemsProvider = ({ children }: { children: React.ReactNode }) => {
  const initialState: ItemsState = {
    items: [],
    recentVideos: [],
  };

  const [state, dispatch] = useReducer(itemsReducer, initialState);

  return (
    <ItemContext.Provider value={{ state, dispatch }}>
      {children}
    </ItemContext.Provider>
  );
};

export const useItemContext = () => {
  const context = useContext(ItemContext);
  if (!context) {
    throw new Error(
      'useItemContext must be used within an ItemContextProvider'
    );
  }
  return context;
};
