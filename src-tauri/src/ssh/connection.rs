use anyhow::Result;
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Authentication method for SSH connections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthMethod {
    Password { password: String },
    Key { key_path: String, passphrase: Option<String> },
    Agent,
}

/// Configuration for a single SSH connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth: AuthMethod,
}

/// Jump host configuration for proxy chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpHostConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth: AuthMethod,
}

/// Full connection config including proxy chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub target: SshConfig,
    pub jump_hosts: Vec<JumpHostConfig>,
    pub keepalive_interval: Option<u64>,
}

/// Represents an active SSH session
pub struct SshSession {
    pub id: String,
    pub config: ConnectionConfig,
    _handle: Option<russh::client::Handle<SshHandler>>,
    channel: Option<russh::Channel<russh::client::Msg>>,
    /// Sender for data going TO the SSH server
    pub tx: mpsc::Sender<Vec<u8>>,
    /// Receiver for data coming FROM the SSH server
    pub rx: mpsc::Receiver<Vec<u8>>,
}

/// Handler for SSH client events
pub struct SshHandler {
    data_tx: mpsc::Sender<Vec<u8>>,
}

#[async_trait]
impl russh::client::Handler for SshHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh_keys::key::PublicKey,
    ) -> std::result::Result<bool, Self::Error> {
        // TODO: implement host key verification (known_hosts)
        // For MVP, accept all keys (NOT safe for production)
        tracing::warn!("Accepting server key without verification - implement known_hosts check");
        Ok(true)
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut russh::client::Session,
    ) -> std::result::Result<(), Self::Error> {
        self.data_tx.send(data.to_vec()).await?;
        Ok(())
    }
}

impl SshSession {
    /// Create a new SSH session (does not connect yet)
    pub fn new(id: String, config: ConnectionConfig) -> Self {
        let (tx, _rx_internal) = mpsc::channel(256);
        let (_tx_internal, rx) = mpsc::channel(256);

        Self {
            id,
            config,
            _handle: None,
            channel: None,
            tx,
            rx,
        }
    }

    /// Establish the SSH connection
    pub async fn connect(&mut self) -> Result<()> {
        let (data_tx, data_rx) = mpsc::channel(256);

        let ssh_config = russh::client::Config::default();
        let handler = SshHandler { data_tx };

        let addr = format!("{}:{}", self.config.target.host, self.config.target.port);
        tracing::info!("Connecting to {}", addr);

        let mut handle =
            russh::client::connect(Arc::new(ssh_config), addr.as_str(), handler).await?;

        // Authenticate
        match &self.config.target.auth {
            AuthMethod::Password { password } => {
                let auth_result = handle
                    .authenticate_password(&self.config.target.username, password)
                    .await?;
                if !auth_result {
                    anyhow::bail!("Password authentication failed");
                }
            }
            AuthMethod::Key { key_path, passphrase } => {
                let key = russh_keys::load_secret_key(key_path, passphrase.as_deref())?;
                let auth_result = handle
                    .authenticate_publickey(&self.config.target.username, Arc::new(key))
                    .await?;
                if !auth_result {
                    anyhow::bail!("Key authentication failed");
                }
            }
            AuthMethod::Agent => {
                // TODO: implement SSH agent authentication
                anyhow::bail!("SSH agent auth not yet implemented");
            }
        }

        // Open a channel and request PTY
        let channel = handle.channel_open_session().await?;
        channel
            .request_pty(
                false,
                "xterm-256color",
                80,
                24,
                0,
                0,
                &[],
            )
            .await?;
        channel.request_shell(false).await?;

        tracing::info!("Connected to {}", self.config.target.host);
        self._handle = Some(handle);
        self.channel = Some(channel);
        self.rx = data_rx;

        Ok(())
    }

    /// Write data to the SSH channel (user input)
    pub async fn write(&self, data: &[u8]) -> Result<()> {
        if let Some(ref channel) = self.channel {
            channel.data(data).await?;
        }
        Ok(())
    }

    /// Resize the PTY
    pub async fn resize(&self, cols: u32, rows: u32) -> Result<()> {
        if let Some(ref channel) = self.channel {
            channel.window_change(cols, rows, 0, 0).await?;
        }
        Ok(())
    }

    /// Disconnect the session
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(channel) = self.channel.take() {
            let _ = channel.eof().await;
        }
        if let Some(handle) = self._handle.take() {
            handle
                .disconnect(russh::Disconnect::ByApplication, "", "en")
                .await?;
        }
        tracing::info!("Disconnected from {}", self.config.target.host);
        Ok(())
    }
}
