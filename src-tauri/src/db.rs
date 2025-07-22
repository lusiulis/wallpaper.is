use once_cell::sync::OnceCell;
use rusqlite::{Connection,params};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

static DB_PATH: OnceCell<PathBuf> = OnceCell::new();

pub fn init_db(app: &AppHandle) -> rusqlite::Result<()> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("No se pudo obtener app_data_dir");
    let db_dir = app_data_dir.join("wallpaper.is");
    std::fs::create_dir_all(&db_dir).expect("No se pudo crear carpeta de datos");
    let db_path = db_dir.join("data.sqlite3");

    let conn = Connection::open(&db_path)?;
    DB_PATH.set(db_path).expect("DB_PATH ya fue inicializado");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            is_folder INTEGER NOT NULL,
            parent TEXT,
            FOREIGN KEY(parent) REFERENCES items(id)
        )",
        [],
    )?;

    Ok(())
}

#[derive(Serialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub value: String,
    pub is_folder: bool,
    pub parent: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Folder {
    pub id: String,
    pub value: String,
    pub children: Vec<Node>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)] // serialize as either a Folder or an Item
pub enum Node {
    Folder(Folder),
    Item(Item),
}

fn get_db_path() -> &'static PathBuf {
    DB_PATH.get().expect("DB_PATH no está inicializado")
}

pub fn add_db_item(item: Item) -> Result<(), String> {
    let conn = Connection::open(get_db_path()).expect("Connection to database failed");
    conn.execute(
        "INSERT INTO items (id, value, is_folder, parent) VALUES (?1, ?2, ?3, ?4)",
        params![
            item.id,
            item.value,
            item.is_folder as i32,
            item.parent,
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_db_items() -> rusqlite::Result<Vec<Item>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare("SELECT id, value, is_folder, parent FROM items")?;

    let item_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            value: row.get(1)?,
            is_folder: row.get(2)?,
            parent: row.get(3)?,
        })
    })?;

    let mut items = Vec::new();
    for item in item_iter {
        items.push(item?);
    }

    Ok(items)
}

pub fn build_tree(items: Vec<Item>) -> Vec<Node> {
    // Mapa de id -> Node (con folders vacíos)
    let mut nodes: HashMap<String, Node> = HashMap::new();

    // Primero convierte todos los folders en Folder con children vacíos
    for item in items.iter() {
        if item.is_folder {
            nodes.insert(
                item.id.clone(),
                Node::Folder(Folder {
                    id: item.id.clone(),
                    value: item.value.clone(),
                    children: Vec::new(),
                }),
            );
        } else {
            // Items simples se insertan como Node::Item
            nodes.insert(item.id.clone(), Node::Item(item.clone()));
        }
    }

    // Mapa para agrupar hijos por id del padre
    let mut children_map: HashMap<Option<String>, Vec<Node>> = HashMap::new();

    for item in items {
        let node = nodes.remove(&item.id).unwrap();
        children_map
            .entry(item.parent.clone())
            .or_default()
            .push(node);
    }

    // Función recursiva para agregar hijos a folders
    fn attach_children(node: Node, children_map: &HashMap<Option<String>, Vec<Node>>) -> Node {
        match node {
            Node::Folder(mut folder) => {
                if let Some(kids) = children_map.get(&Some(folder.id.clone())) {
                    folder.children = kids
                        .iter()
                        .map(|child| attach_children(child.clone(), children_map))
                        .collect();
                }
                Node::Folder(folder)
            }
            node @ Node::Item(_) => node,
        }
    }

    // Los nodos raíz tienen parent = None
    let empty = Vec::new();
    let roots = children_map.get(&None).unwrap_or(&empty);

    roots
        .iter()
        .map(|node| attach_children(node.clone(), &children_map))
        .collect()
}
