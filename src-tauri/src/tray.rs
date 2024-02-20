use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
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
        .add_item(hide);

    // 设置在右键单击系统托盘时显示菜单
    return SystemTray::new().with_menu(tray_menu);
}
// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    let parent_window = Some(&window);
    // 匹配点击事件
    match event {
        SystemTrayEvent::LeftClick {
            tray_id,
            position,
            size,
            ..
        } => {
            println!(
                "LeftClick: tray_id: {}, position: {:?}, size: {:?}",
                tray_id, position, size
            );
        }
        SystemTrayEvent::RightClick {
            tray_id,
            position,
            size,
            ..
        } => {
            println!(
                "RightClick: tray_id: {}, position: {:?}, size: {:?}",
                tray_id, position, size
            );
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);

            match id.as_str() {
                "quit" => {
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
                _ => {}
            }
        }
        _ => {}
    }
}
