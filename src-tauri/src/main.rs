// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use tauri::{Manager, PhysicalPosition, PhysicalSize, Position, Size, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod cmds;
mod tray;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let main_window = app.get_window("main");
            let window = main_window.unwrap();

            // 读取并设置窗口大小: std::path::PathBuf和位置的信息
            let stores = app.state::<StoreCollection<Wry>>();
            let mut data = app.path_resolver().resource_dir().unwrap();
            data.push("data/settings.json");

            // 窗口大小和位置信息
            let mut win_w: u32 = 0;
            let mut win_h: u32 = 0;
            let mut pos_x: i32 = 0;
            let mut pos_y: i32 = 0;

            let mut click_through: bool = false;
            let mut stay_top: bool = true;
            let mut is_mmd: bool = false;

            with_store(app.app_handle(), stores, data, |store| {
                win_w = match store.get("win_w") {
                    Some(v) => v.as_u64().unwrap() as u32,
                    None => 2200,
                };
                win_h = match store.get("win_h") {
                    None => 150,
                    Some(v) => v.as_u64().unwrap() as u32,
                };
                pos_x = match store.get("pos_x") {
                    Some(v) => v.as_i64().unwrap() as i32,
                    None => 10,
                };
                pos_y = match store.get("pos_y") {
                    Some(v) => v.as_i64().unwrap() as i32,
                    None => 900,
                };
                click_through = match store.get("click_through") {
                    Some(v) => v.as_bool().unwrap(),
                    None => false,
                };
                stay_top = match store.get("stay_top") {
                    Some(v) => v.as_bool().unwrap(),
                    None => true,
                };
                is_mmd = match store.get("is_mmd") {
                    Some(v) => v.as_bool().unwrap(),
                    None => false,
                };
                println!("{is_mmd}");
                store.save();
                Ok(())
            })
            .unwrap();

            // 设置其他信息
            window.set_ignore_cursor_events(click_through).unwrap();
            window.set_always_on_top(stay_top).unwrap();
            println!("is_mmd: {}", win_w);
            if is_mmd {
                let size = Size::Physical(PhysicalSize {
                    width: win_w * 3,
                    height: win_h * 6,
                });
                let pos = Position::Physical(PhysicalPosition {
                    x: pos_x,
                    y: pos_y - 700,
                });
                window.set_size(size).unwrap();
                window.set_position(pos).unwrap();
            }
            // 设置系统托盘

            // 打开控制台窗口
            window.open_devtools();
            Ok(())
        })
        .system_tray(
            tray::menu(), // tauri::SystemTray::new()
                          // .icon("icon.png")
                          // .menu(&[
                          //     tauri::SystemTrayMenuItem::normal("Open", "Open the app"),
                          //     tauri::SystemTrayMenuItem::normal("Quit", "Quit the app"),
                          // ])
                          // .on_menu_item_select()
        )
        .on_system_tray_event(tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
