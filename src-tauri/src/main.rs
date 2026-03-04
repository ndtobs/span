// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports)]

use tauri::Manager;

mod commands;
mod credentials;
mod inventory;
mod scripting;
mod ssh;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // SSH commands
            commands::ssh::connect,
            commands::ssh::disconnect,
            commands::ssh::write_data,
            commands::ssh::resize,
            // Inventory commands
            commands::inventory::list_devices,
            commands::inventory::add_device,
            commands::inventory::update_device,
            commands::inventory::delete_device,
            commands::inventory::add_folder,
            commands::inventory::import_securecrt,
            // Credential commands
            commands::credentials::list_credentials,
            commands::credentials::store_credential,
            commands::credentials::delete_credential,
        ])
        .setup(|app| {
            // Initialize database
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");

            let db_path = app_dir.join("span.db");
            inventory::db::initialize(&db_path).expect("Failed to initialize database");

            tracing::info!("Span initialized, db at {:?}", db_path);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Span");
}
