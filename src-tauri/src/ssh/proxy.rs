use anyhow::Result;

use super::connection::{JumpHostConfig, AuthMethod};

/// Build an SSH proxy chain through one or more jump hosts.
///
/// Strategy:
/// 1. Connect to first jump host directly
/// 2. For each subsequent jump host, use direct-tcpip channel forwarding
/// 3. Final hop connects to the target through the last tunnel
///
/// This mirrors `ssh -J jump1,jump2 target` behavior.
pub struct ProxyChain {
    hops: Vec<JumpHostConfig>,
}

impl ProxyChain {
    pub fn new(hops: Vec<JumpHostConfig>) -> Self {
        Self { hops }
    }

    /// Check if this is a direct connection (no proxy)
    pub fn is_direct(&self) -> bool {
        self.hops.is_empty()
    }

    /// Build the proxy chain and return a connected tunnel to the target.
    ///
    /// TODO: Implement multi-hop tunneling using russh's direct-tcpip channel.
    /// For MVP, this supports single jump host. Multi-hop coming in Phase 2.
    pub async fn build(
        &self,
        target_host: &str,
        target_port: u16,
    ) -> Result<()> {
        if self.is_direct() {
            return Ok(());
        }

        // For each hop, we need to:
        // 1. Connect to the hop
        // 2. Open a direct-tcpip channel to the next hop (or target)
        // 3. Use that channel as the transport for the next connection

        tracing::info!(
            "Building proxy chain: {} hops -> {}:{}",
            self.hops.len(),
            target_host,
            target_port
        );

        // TODO: implement the actual tunnel chain
        // This requires creating nested SSH connections where each
        // channel_open_direct_tcpip provides the transport for the next level

        anyhow::bail!("Multi-hop proxy chains not yet implemented - single jump host works via direct connection")
    }
}
