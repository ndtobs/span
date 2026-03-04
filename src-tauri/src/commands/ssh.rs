use tauri::command;
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};

use crate::ssh::{manager::SshManager, connection::{ConnectionConfig, SshConfig, AuthMethod}};
use tokio::sync::mpsc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

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
    state: State<'_, SshManager>,
    app_handle: AppHandle,
    args: ConnectArgs,
) -> Result<ConnectResult, String> {
    tracing::info!("SSH connect: {}@{}:{}", args.username, args.host, args.port);

    let auth = match args.auth_method.as_str() {
        "password" => {
            let password = args.password.ok_or_else(|| "Password required for password auth".to_string())?;
            AuthMethod::Password { password }
        }
        "key" => {
            let key_path = args.key_path.ok_or_else(|| "Key path required for key auth".to_string())?;
            AuthMethod::Key { key_path, passphrase: None }
        }
        _ => return Ok(ConnectResult {
            session_id: args.session_id.clone(),
            success: false,
            error: Some("Unsupported auth_method: use 'password' or 'key'".to_string()),
        }),
    };

    let config = ConnectionConfig {
        target: SshConfig {
            host: args.host,
            port: args.port,
            username: args.username,
            auth,
        },
        jump_hosts: vec![], // TODO: wire jump hosts
        keepalive_interval: None,
    };

    let rx = state.connect(args.session_id.clone(), config).await
        .map_err(|e| format!("Connect failed: {}", e))?;

    let app_handle = app_handle.clone();
    let session_id = args.session_id.clone();

    tauri::async_runtime::spawn(async move {
        while let Some(data) = rx.recv().await {
            let payload = STANDARD.encode(&data);
            let _ = app_handle.emit(format!("session-data-{}", session_id), payload);
        }
        tracing::debug!("Data forwarding ended for session {}", session_id);
    });

    Ok(ConnectResult {
        session_id: args.session_id,
        success: true,
        error: None,
    })
}

/// Disconnect an SSH session
#[command]
pub async fn disconnect(state: State<'_, SshManager>, session_id: String) -> Result<(), String> {
    tracing::info!("SSH disconnect: {}", session_id);
    state.disconnect(&session_id).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Write data to an SSH session (user keyboard input)
#[command]
pub async fn write_data(state: State<'_, SshManager>, session_id: String, data: String) -> Result<(), String> {
    state.write(&session_id, data.as_bytes()).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Resize the PTY for an SSH session
#[command]
pub async fn resize(state: State<'_, SshManager>, session_id: String, cols: u32, rows: u32) -> Result<(), String> {
    tracing::debug!("Resize {}: {}x{}", session_id, cols, rows);
    state.resize(&session_id, cols, rows).await.map_err(|e| e.to_string())?;
    Ok(())
}
