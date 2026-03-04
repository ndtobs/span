use tauri::command;
use serde::{Deserialize, Serialize};

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
pub async fn connect(args: ConnectArgs) -> Result<ConnectResult, String> {
    tracing::info!("SSH connect: {}@{}:{}", args.username, args.host, args.port);

    // TODO: wire to SshManager
    // 1. Build ConnectionConfig from args
    // 2. Call ssh_manager.connect()
    // 3. Set up event forwarding (SSH data → frontend via Tauri events)

    Ok(ConnectResult {
        session_id: args.session_id,
        success: true,
        error: None,
    })
}

/// Disconnect an SSH session
#[command]
pub async fn disconnect(session_id: String) -> Result<(), String> {
    tracing::info!("SSH disconnect: {}", session_id);
    // TODO: ssh_manager.disconnect(&session_id)
    Ok(())
}

/// Write data to an SSH session (user keyboard input)
#[command]
pub async fn write_data(_session_id: String, _data: String) -> Result<(), String> {
    // TODO: ssh_manager.write(&session_id, data.as_bytes())
    Ok(())
}

/// Resize the PTY for an SSH session
#[command]
pub async fn resize(session_id: String, cols: u32, rows: u32) -> Result<(), String> {
    tracing::debug!("Resize {}: {}x{}", session_id, cols, rows);
    // TODO: ssh_manager.resize(&session_id, cols, rows)
    Ok(())
}
