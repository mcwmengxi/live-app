// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use tauri::{Manager, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod tray;
fn main() {
    tauri::Builder::default()
        .system_tray(
            tray::menu(), // tauri::SystemTray::new()
                          // .icon("icon.png")
                          // .menu(&[
                          //     tauri::SystemTrayMenuItem::normal("Open", "Open the app"),
                          //     tauri::SystemTrayMenuItem::normal("Quit", "Quit the app"),
                          // ])
                          // .on_menu_item_select()
        )
        .on_system_tray_event(tray::handler)
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let stores = app.state::<StoreCollection<Wry>>();
            let mut data = app.path_resolver().resource_dir().unwrap();
            data.push("data/settings.json");
            // dbg!("data dir: {}", data);

            with_store(app.app_handle(), stores, data, |store| {
                store.insert("a".to_string(), json!("b"))
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
