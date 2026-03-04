use tauri::command;
use serde::{Deserialize, Serialize};
use serde_json;
use rusqlite::{params, Result as SqlResult};
use crate::inventory::{db, Device, Folder};

#[derive(Debug, Serialize)]
pub struct DeviceListResponse {
    pub folders: Vec<inventory::Folder>,
    pub devices: Vec<inventory::Device>,
}

/// List all devices and folders
#[command]
pub async fn list_devices() -> Result<DeviceListResponse, String> {
    let db = db::get_db().lock().map_err(|_| "DB lock failed".to_string())?;

    let mut stmt = db.prepare("SELECT id, name, parent_id, sort_order FROM folders ORDER BY COALESCE(parent_id, ''), sort_order ASC")?;
    let folders_rows: SqlResult<Vec<(String, String, Option<String>, i32)>> = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?.collect();
    let folders = folders_rows.map_err(|e| e.to_string())?.into_iter().map(|(id, name, parent_id, sort_order)| Folder {
        id, name, parent_id, sort_order
    }).collect();

    let mut stmt = db.prepare("
        SELECT id, name, folder_id, host, port, username, auth_method, key_path, platform, 
               tags, jump_hosts, post_connect_commands, notes, last_connected, created_at, updated_at 
        FROM devices
    ")?;
    let devices_rows: SqlResult<Vec<Device>> = stmt.query_map([], |row| {
        let tags_json: String = row.get(9)?;
        let jump_hosts_json: String = row.get(10)?;
        let post_cmds_json: String = row.get(11)?;
        Ok(Device {
            id: row.get(0)?,
            name: row.get(1)?,
            folder_id: row.get(2)?,
            host: row.get(3)?,
            port: row.get(4)?,
            username: row.get(5)?,
            auth_method: row.get(6)?,
            key_path: row.get(7)?,
            platform: row.get(8)?,
            tags: serde_json::from_str(&tags_json).unwrap_or_default(),
            jump_hosts: serde_json::from_str(&jump_hosts_json).unwrap_or_default(),
            post_connect_commands: serde_json::from_str(&post_cmds_json).unwrap_or_default(),
            notes: row.get(12)?,
            last_connected: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    })?.collect();
    let devices = devices_rows.map_err(|e| e.to_string())?;

    Ok(DeviceListResponse { folders, devices })
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
pub async fn add_device(args: AddDeviceArgs) -> Result<Device, String> {
    let now = chrono::Utc::now().timestamp();
    let id = uuid::Uuid::new_v4().to_string();
    let tags_json = serde_json::to_string(&args.tags.unwrap_or_default()).map_err(|e| e.to_string())?;
    let jump_json = serde_json::to_string(&vec![]).map_err(|e| e.to_string())?;
    let post_json = serde_json::to_string(&vec![]).map_err(|e| e.to_string())?;

    let device = Device {
        id: id.clone(),
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
        created_at: now,
        updated_at: now,
    };

    let db = db::get_db().lock().map_err(|_| "DB lock failed".to_string())?;
    db.execute(
        "INSERT INTO devices (id, name, folder_id, host, port, username, auth_method, key_path, platform, tags, jump_hosts, post_connect_commands, notes, created_at, updated_at) 
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
        params![
            &id,
            &device.name,
            &device.folder_id,
            &device.host,
            &device.port,
            &device.username,
            &device.auth_method,
            &device.key_path,
            &device.platform,
            &tags_json,
            &jump_json,
            &post_json,
            &device.notes,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(device)
}

/// Update an existing device
#[command]
pub async fn update_device(id: String, _args: AddDeviceArgs) -> Result<(), String> {
    tracing::info!("Update device: {}", id);
    // TODO: update in SQLite
    Ok(())
}

/// Delete a device
#[command]
pub async fn delete_device(id: String) -> Result<(), String> {
    tracing::info!("Delete device: {}", id);
    let db = db::get_db().lock().map_err(|_| "DB lock failed".to_string())?;
    db.execute("DELETE FROM devices WHERE id = ?1", [&id]).map_err(|e| e.to_string())?;
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
