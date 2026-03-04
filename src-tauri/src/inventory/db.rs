use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

/// Initialize the SQLite database with schema
pub fn initialize(db_path: &Path) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch("
        PRAGMA journal_mode=WAL;
        PRAGMA foreign_keys=ON;

        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT REFERENCES folders(id) ON DELETE CASCADE,
            sort_order INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch())
        );

        CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            folder_id TEXT REFERENCES folders(id) ON DELETE SET NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL DEFAULT 22,
            username TEXT NOT NULL DEFAULT '',
            auth_method TEXT NOT NULL DEFAULT 'key',
            key_path TEXT,
            platform TEXT,
            tags TEXT DEFAULT '[]',
            jump_hosts TEXT DEFAULT '[]',
            post_connect_commands TEXT DEFAULT '[]',
            notes TEXT,
            last_connected INTEGER,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch())
        );

        CREATE TABLE IF NOT EXISTS credentials (
            id TEXT PRIMARY KEY,
            label TEXT NOT NULL,
            username TEXT NOT NULL,
            password_ref TEXT,
            key_path TEXT,
            created_at INTEGER NOT NULL DEFAULT (unixepoch()),
            updated_at INTEGER NOT NULL DEFAULT (unixepoch())
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_devices_folder ON devices(folder_id);
        CREATE INDEX IF NOT EXISTS idx_devices_name ON devices(name);
        CREATE INDEX IF NOT EXISTS idx_folders_parent ON folders(parent_id);
    ")?;

    DB.set(Mutex::new(conn))
        .map_err(|_| anyhow::anyhow!("Database already initialized"))?;

    tracing::info!("Database initialized at {:?}", db_path);
    Ok(())
}

/// Get a reference to the database connection
pub fn get_db() -> &'static Mutex<Connection> {
    DB.get().expect("Database not initialized - call initialize() first")
}
