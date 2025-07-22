type BaseItem = {
  id: string;
  value: string;
};

type Item = BaseItem & {
  is_folder: false;
  parent: string | null;
};

type Folder = BaseItem & {
  children: TreeNode[];
};

type TreeNode = Folder | Item;
