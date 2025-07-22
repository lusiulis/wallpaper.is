use crate::db::{add_db_item, Folder, Item};
use uuid::Uuid;

#[tauri::command]
pub fn add_folder(value: &str, parent: Option<String>) -> Result<Folder, String> {
    let new_item = Item {
        id: Uuid::new_v4().to_string(),
        value: value.to_string(),
        is_folder: true,
        parent,
    };

    add_db_item(new_item.clone())?;

    let folder = Folder {
        id: new_item.id,
        value: new_item.value,
        children: Vec::new(),
    };
    Ok(folder)
}
