use crate::cmds;
use serde_json::json;
use tauri::{
    AppHandle, CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu,
};
use tauri_plugin_store::{with_store, StoreCollection};
pub mod tray {
    // 在这里定义你的公共结构体、枚举、函数等...
}
pub struct Tray {}

impl Tray {
    pub fn tray_menu(app_handle: &AppHandle) -> SystemTrayMenu {
        let quit = CustomMenuItem::new("quit".to_string(), "退出");
        let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
        let click_through = CustomMenuItem::new("click_through".to_string(), "点击穿透");
        let tray_menu = SystemTrayMenu::new()
            .add_submenu(SystemTraySubmenu::new(
                "File",
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("new file".to_string(), "NewFile"))
                    .add_item(CustomMenuItem::new("edit file".to_string(), "EditFile")),
            ))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(hide)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(click_through)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(quit)
            .accelerator("CmdOrControl+Q");
    }
    pub fn on_left_click(
        app: &AppHandle,
        tray_id: String,
        position: PhysicalPosition<f64>,
        size: PhysicalSize<f64>,
    ) {
        println!(
            "LeftClick: tray_id: {}, position: {:?}, size: {:?}",
            tray_id, position, size
        );
    }
    pub fn on_right_click(
        app: &AppHandle,
        tray_id: String,
        position: PhysicalPosition<f64>,
        size: PhysicalSize<f64>,
    ) {
        println!(
            "RightClick: tray_id: {}, position: {:?}, size: {:?}",
            tray_id, position, size
        );
    }

    // 菜单事件
    pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
        // 获取应用窗口
        let window = app.get_window("main").unwrap();
        let parent_window = Some(&window);

        let stores = app.state::<StoreCollection<Wry>>();
        let mut data = app.path_resolver().resource_dir().unwrap();
        data.push("data/settings.json");

        // 匹配点击事件
        match event {
            SystemTrayEvent::LeftClick {
                tray_id,
                position,
                size,
                ..
            } => Tray::on_left_click(app, tray_id, position, size),
            SystemTrayEvent::RightClick {
                tray_id,
                position,
                size,
                ..
            } => Tray::on_right_click(app, tray_id, position, size),
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);

                match id.as_str() {
                    "quit" => {
                        cmds::exit_app(app.clone());
                        std::process::exit(0);
                        // app.exit(0);
                    }
                    "hide" => {
                        if !window.is_visible().unwrap() {
                            window.show().unwrap();
                            item_handle.set_title("隐藏").unwrap();
                        } else {
                            window.hide().unwrap();
                            item_handle.set_title("显示").unwrap();
                        }
                    }
                    "click_through" => {
                        let mut click_through = false;

                        with_store(app.app_handle(), stores, data, |store| {
                            click_through = match store.get("click_through") {
                                Some(v) => v.as_bool().unwrap(),
                                None => false,
                            };
                            store
                                .insert("click_through".to_string(), json!(!click_through))
                                .unwrap();
                            store.save();
                            Ok(())
                        })
                        .unwrap();
                        window.set_ignore_cursor_events(!click_through).unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

// 托盘菜单
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let click_through = CustomMenuItem::new("click_through".to_string(), "点击穿透");
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "File",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("new file".to_string(), "NewFile"))
                .add_item(CustomMenuItem::new("edit file".to_string(), "EditFile")),
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(click_through);

    // 设置在右键单击系统托盘时显示菜单

    return SystemTray::new().with_menu(tray_menu);
}
