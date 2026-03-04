use tauri::command;
use serde::{Deserialize, Serialize};
use crate::credentials::CredentialVault;

#[derive(Debug, Serialize)]
pub struct CredentialInfo {
    pub id: String,
    pub label: String,
    pub username: String,
    pub has_password: bool,
    pub key_path: Option<String>,
}

/// List all stored credentials (without passwords)
#[command]
pub async fn list_credentials() -> Result<Vec<CredentialInfo>, String> {
    // TODO: query from SQLite, check keyring for password existence
    Ok(vec![])
}

#[derive(Debug, Deserialize)]
pub struct StoreCredentialArgs {
    pub label: String,
    pub username: String,
    pub password: Option<String>,
    pub key_path: Option<String>,
}

/// Store a new credential
#[command]
pub async fn store_credential(args: StoreCredentialArgs) -> Result<CredentialInfo, String> {
    let id = uuid::Uuid::new_v4().to_string();

    // Store password in OS keyring if provided
    if let Some(ref password) = args.password {
        CredentialVault::store_password(&id, password)
            .map_err(|e| format!("Failed to store in keyring: {}", e))?;
    }

    // TODO: store metadata in SQLite

    Ok(CredentialInfo {
        id,
        label: args.label,
        username: args.username,
        has_password: args.password.is_some(),
        key_path: args.key_path,
    })
}

/// Delete a credential
#[command]
pub async fn delete_credential(id: String) -> Result<(), String> {
    // Delete from keyring
    let _ = CredentialVault::delete_password(&id);

    // TODO: delete from SQLite

    Ok(())
}
