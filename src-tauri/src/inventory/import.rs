use anyhow::Result;
use std::path::Path;

/// Import sessions from SecureCRT XML session files.
///
/// SecureCRT stores sessions as .ini files in a directory structure like:
///   Sessions/
///     folder1/
///       device1.ini
///       device2.ini
///     folder2/
///       device3.ini
///
/// Each .ini file contains connection parameters like:
///   S:"Hostname"=192.168.1.1
///   D:"Port"=00000016 (hex for 22)
///   S:"Username"=admin
///   S:"Firewall Name"=jump-host-session-name
///
/// TODO: Implement full parser for SecureCRT session format
pub fn import_securecrt_sessions(sessions_dir: &Path) -> Result<Vec<ImportedSession>> {
    if !sessions_dir.exists() {
        anyhow::bail!("SecureCRT sessions directory not found: {:?}", sessions_dir);
    }

    let mut sessions = Vec::new();

    // Walk the directory tree
    walk_sessions_dir(sessions_dir, &mut sessions, None)?;

    tracing::info!("Imported {} sessions from SecureCRT", sessions.len());
    Ok(sessions)
}

fn walk_sessions_dir(
    dir: &Path,
    sessions: &mut Vec<ImportedSession>,
    folder: Option<String>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            walk_sessions_dir(&path, sessions, Some(folder_name))?;
        } else if path.extension().map_or(false, |ext| ext == "ini") {
            if let Ok(session) = parse_securecrt_ini(&path, &folder) {
                sessions.push(session);
            }
        }
    }
    Ok(())
}

fn parse_securecrt_ini(path: &Path, folder: &Option<String>) -> Result<ImportedSession> {
    let content = std::fs::read_to_string(path)?;

    let mut session = ImportedSession {
        name: path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        folder: folder.clone(),
        host: String::new(),
        port: 22,
        username: String::new(),
        protocol: "ssh".to_string(),
    };

    for line in content.lines() {
        let line = line.trim();

        if let Some(value) = extract_string_value(line, "Hostname") {
            session.host = value;
        }
        if let Some(value) = extract_string_value(line, "Username") {
            session.username = value;
        }
        if let Some(value) = extract_hex_value(line, "Port") {
            session.port = value;
        }
        if let Some(value) = extract_string_value(line, "Protocol Name") {
            session.protocol = value.to_lowercase();
        }
    }

    if session.host.is_empty() {
        anyhow::bail!("No hostname found in {:?}", path);
    }

    Ok(session)
}

fn extract_string_value(line: &str, key: &str) -> Option<String> {
    let prefix = format!("S:\"{}\"=", key);
    if line.starts_with(&prefix) {
        Some(line[prefix.len()..].to_string())
    } else {
        None
    }
}

fn extract_hex_value(line: &str, key: &str) -> Option<u16> {
    let prefix = format!("D:\"{}\"=", key);
    if line.starts_with(&prefix) {
        let hex_str = &line[prefix.len()..];
        u32::from_str_radix(hex_str, 16).ok().map(|v| v as u16)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ImportedSession {
    pub name: String,
    pub folder: Option<String>,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub protocol: String,
}
