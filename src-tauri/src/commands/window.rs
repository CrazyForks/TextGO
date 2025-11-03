use tauri::{Emitter, Manager};

#[tauri::command]
pub fn show_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn hide_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        // 隐藏窗口时，隐藏 Dock 图标
        #[cfg(target_os = "macos")]
        {
            let _ = app.set_dock_visibility(false);
        }
    }
}

#[tauri::command]
pub fn toggle_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                // 窗口可见，检查是否聚焦
                match window.is_focused() {
                    Ok(true) => {
                        // 窗口可见且聚焦，调用 hide_window
                        hide_window(app);
                    }
                    Ok(false) | Err(_) => {
                        // 窗口可见但未聚焦，把窗口置于前端并聚焦
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                }
            }
            Ok(false) | Err(_) => {
                // 窗口隐藏或出错，调用 show_window
                show_window(app);
            }
        }
    }
}

#[tauri::command]
pub fn goto_shortcuts(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        // 发送页面跳转事件
        let _ = window.emit("goto-shortcuts", ());
    }
}
