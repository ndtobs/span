use tauri::command;
use serde::{Deserialize, Serialize};
use crate::inventory;

#[derive(Debug, Serialize)]
pub struct DeviceListResponse {
    pub folders: Vec<inventory::Folder>,
    pub devices: Vec<inventory::Device>,
}

/// List all devices and folders
#[command]
pub async fn list_devices() -> Result<DeviceListResponse, String> {
    // TODO: query from SQLite
    Ok(DeviceListResponse {
        folders: vec![],
        devices: vec![],
    })
}

#[derive(Debug, Deserialize)]
pub struct AddDeviceArgs {
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub auth_method: Option<String>,
    pub key_path: Option<String>,
    pub folder_id: Option<String>,
    pub platform: Option<String>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
}

/// Add a new device to inventory
#[command]
pub async fn add_device(args: AddDeviceArgs) -> Result<inventory::Device, String> {
    let device = inventory::Device {
        id: uuid::Uuid::new_v4().to_string(),
        name: args.name,
        folder_id: args.folder_id,
        host: args.host,
        port: args.port.unwrap_or(22),
        username: args.username,
        auth_method: args.auth_method.unwrap_or_else(|| "key".to_string()),
        key_path: args.key_path,
        platform: args.platform,
        tags: args.tags.unwrap_or_default(),
        jump_hosts: vec![],
        post_connect_commands: vec![],
        notes: args.notes,
        last_connected: None,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };

    // TODO: insert into SQLite

    Ok(device)
}

/// Update an existing device
#[command]
pub async fn update_device(id: String, args: AddDeviceArgs) -> Result<(), String> {
    tracing::info!("Update device: {}", id);
    // TODO: update in SQLite
    Ok(())
}

/// Delete a device
#[command]
pub async fn delete_device(id: String) -> Result<(), String> {
    tracing::info!("Delete device: {}", id);
    // TODO: delete from SQLite
    Ok(())
}

/// Add a new folder
#[command]
pub async fn add_folder(name: String, parent_id: Option<String>) -> Result<inventory::Folder, String> {
    let folder = inventory::Folder {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        parent_id,
        sort_order: 0,
    };

    // TODO: insert into SQLite

    Ok(folder)
}

/// Import sessions from SecureCRT
#[command]
pub async fn import_securecrt(path: String) -> Result<u32, String> {
    let sessions = inventory::import::import_securecrt_sessions(std::path::Path::new(&path))
        .map_err(|e| e.to_string())?;

    let count = sessions.len() as u32;

    // TODO: convert ImportedSession → Device and insert into SQLite

    tracing::info!("Imported {} sessions from SecureCRT", count);
    Ok(count)
}
