use crate::db::{build_tree, get_db_items, Node};

#[tauri::command]
pub fn get_items() -> Result<Vec<Node>, String> {
    let items = get_db_items()
        .map_err(|e| e.to_string())
        .expect("Couldn't get items from database");
    Ok(build_tree(items))
}
