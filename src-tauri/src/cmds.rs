use tauri::api;

#[cfg(windows)]
pub mod cmds {
    // 在这里定义你的公共结构体、枚举、函数等...
}

#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle) {
    // exit_app
    api::process::kill_children();
    app_handle.exit(0);
    std::process::exit(0);
}
