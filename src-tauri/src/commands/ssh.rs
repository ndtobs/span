use tauri::command;
use tauri::{AppHandle, State, Emitter};
use serde::Serialize;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::ssh::{manager::SshManager, connection::{ConnectionConfig, SshConfig, AuthMethod}};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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
    session_id: String,
    host: String,
    port: u16,
    username: String,
    auth_method: Option<String>,
    password: Option<String>,
    key_path: Option<String>,
) -> Result<ConnectResult, String> {
    tracing::info!("SSH connect: {}@{}:{}", username, host, port);

    let auth = match auth_method.as_deref().unwrap_or("password") {
        "key" => AuthMethod::Key { key_path: key_path.unwrap_or_default(), passphrase: None },
        _ => AuthMethod::Password { password: password.unwrap_or_default() },
    };

    let config = ConnectionConfig {
        target: SshConfig {
            host,
            port,
            username,
            auth,
        },
        jump_hosts: vec![], // TODO: handle jump hosts
        keepalive_interval: None,
    };

    let session_id = session_id.clone();

    // Connect — returns the real rx channel with SSH output data
    let mut rx = ssh_manager.connect(session_id.clone(), config).await.map_err(|e| {
        let _ = app_handle.emit(&format!("session-status-{}", session_id), "error");
        e.to_string()
    })?;

    // Emit connected status
    let _ = app_handle.emit(&format!("session-status-{}", session_id), "connected");

    // Spawn task to forward SSH data as Tauri events
    let app_handle_clone = app_handle.clone();
    let session_id_for_emit = session_id.clone();

    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            let payload = STANDARD.encode(&data);
            let _ = app_handle_clone.emit(&format!("session-data-{}", session_id_for_emit), payload);
        }
        // Channel closed — session disconnected
        let _ = app_handle_clone.emit(&format!("session-status-{}", session_id_for_emit), "disconnected");
    });

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
