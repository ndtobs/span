use tauri::command;
use tauri::{AppHandle, State, Emitter};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::ssh::{manager::SshManager, connection::{ConnectionConfig, SshConfig, AuthMethod}};

#[derive(Debug, Deserialize)]
pub struct ConnectArgs {
    pub session_id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_method: String,
    pub password: Option<String>,
    pub key_path: Option<String>,
    pub jump_host: Option<String>,
    pub jump_port: Option<u16>,
    pub jump_username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConnectResult {
    pub session_id: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Initiate an SSH connection
#[command]
pub async fn connect(
    app_handle: AppHandle,
    ssh_manager: State<'_, SshManager>,
    args: ConnectArgs
) -> Result<ConnectResult, String> {
    tracing::info!("SSH connect: {}@{}:{}", args.username, args.host, args.port);

    let config = ConnectionConfig {
        target: SshConfig {
            host: args.host,
            port: args.port,
            username: args.username,
            auth: match args.auth_method.as_str() {

            "password" => AuthMethod::Password { password: args.password.unwrap_or_default() },

            "key" => AuthMethod::Key { key_path: args.key_path.unwrap_or_default() },

            _ => AuthMethod::Password { password: args.password.unwrap_or_default() },

        },
        },
        jump_hosts: vec![], // TODO: handle jump hosts
        keepalive_interval: None,
    };

    // Spawn task to forward SSH data as Tauri events
    let session_id = args.session_id.clone();
    let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(256);
    let app_handle_clone = app_handle.clone();
    let session_id_for_emit = session_id.clone();

    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            let payload = STANDARD.encode(&data);
            let _ = app_handle_clone.emit(&format!("session-data-{}", session_id_for_emit), payload);
        }
    });

    // Connect and store the tx channel
    ssh_manager.connect(session_id.clone(), config).await.map_err(|e| e.to_string())?;

    Ok(ConnectResult {
        session_id,
        success: true,
        error: None,
    })
}

/// Disconnect an SSH session
#[command]
pub async fn disconnect(
    ssh_manager: State<'_, SshManager>,
    session_id: String
) -> Result<(), String> {
    tracing::info!("SSH disconnect: {}", session_id);
    ssh_manager.disconnect(&session_id).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Write data to an SSH session (user keyboard input)
#[command]
pub async fn write_data(
    ssh_manager: State<'_, SshManager>,
    session_id: String,
    data: String
) -> Result<(), String> {
    ssh_manager.write(&session_id, data.as_bytes()).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Resize the PTY for an SSH session
#[command]
pub async fn resize(
    ssh_manager: State<'_, SshManager>,
    session_id: String,
    cols: u32,
    rows: u32
) -> Result<(), String> {
    tracing::debug!("Resize {}: {}x{}", session_id, cols, rows);
    ssh_manager.resize(&session_id, cols, rows).await.map_err(|e| e.to_string())?;
    Ok(())
}
