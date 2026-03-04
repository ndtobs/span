use anyhow::Result;
use keyring::Entry;

const SERVICE_NAME: &str = "span-ssh-terminal";

/// Credential vault using the OS keyring for secure storage.
///
/// Passwords and passphrases are stored in the system keyring
/// (macOS Keychain, Windows Credential Manager, Linux Secret Service).
/// Only references (IDs) are stored in SQLite.
pub struct CredentialVault;

impl CredentialVault {
    /// Store a password in the OS keyring
    pub fn store_password(credential_id: &str, password: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, credential_id)?;
        entry.set_password(password)?;
        tracing::debug!("Stored credential: {}", credential_id);
        Ok(())
    }

    /// Retrieve a password from the OS keyring
    pub fn get_password(credential_id: &str) -> Result<String> {
        let entry = Entry::new(SERVICE_NAME, credential_id)?;
        let password = entry.get_password()?;
        Ok(password)
    }

    /// Delete a password from the OS keyring
    pub fn delete_password(credential_id: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, credential_id)?;
        entry.delete_credential()?;
        tracing::debug!("Deleted credential: {}", credential_id);
        Ok(())
    }

    /// Check if a credential exists in the keyring
    pub fn has_password(credential_id: &str) -> bool {
        Entry::new(SERVICE_NAME, credential_id)
            .and_then(|e| e.get_password())
            .is_ok()
    }
}
