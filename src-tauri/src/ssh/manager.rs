use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

use super::connection::{SshSession, ConnectionConfig};
use tokio::sync::mpsc;

/// Manages all active SSH sessions
pub struct SshManager {
    sessions: Arc<Mutex<HashMap<String, SshSession>>>,
}

impl SshManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create and connect a new SSH session
    pub async fn connect(&self, id: String, config: ConnectionConfig) -> Result<mpsc::Receiver<Vec<u8>>> {
        let mut session = SshSession::new(id.clone(), config);
        session.connect().await?;

        let rx = session.rx.take().expect("No rx channel after connect");
        let mut sessions = self.sessions.lock().await;
        sessions.insert(id, session);
        Ok(rx)
    }

    /// Disconnect and remove a session
    pub async fn disconnect(&self, id: &str) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(mut session) = sessions.remove(id) {
            session.disconnect().await?;
        }
        Ok(())
    }

    /// Write data to a session
    pub async fn write(&self, id: &str, data: &[u8]) -> Result<()> {
        let sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get(id) {
            session.write(data).await?;
        }
        Ok(())
    }

    /// Resize a session's PTY
    pub async fn resize(&self, id: &str, cols: u32, rows: u32) -> Result<()> {
        let sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get(id) {
            session.resize(cols, rows).await?;
        }
        Ok(())
    }

    /// List active session IDs
    pub async fn list(&self) -> Vec<String> {
        let sessions = self.sessions.lock().await;
        sessions.keys().cloned().collect()
    }
}
