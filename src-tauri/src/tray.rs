use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示");
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
        .add_item(hide)
        .add_item(show);

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
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                    // app.exit(0);
                }
                "show" => {
                    window.show().unwrap();
                }
                "hide" => {
                    window.hide().unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}
