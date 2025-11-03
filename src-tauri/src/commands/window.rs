use tauri::{Emitter, Manager, WebviewWindow};

/// 显示主窗口
#[tauri::command]
pub fn show_main_window(app: tauri::AppHandle) {
    show_window(&app, "main");
}

/// 隐藏主窗口
#[tauri::command]
pub fn hide_main_window(app: tauri::AppHandle) {
    hide_window(&app, "main");
}

/// 切换主窗口显示状态
#[tauri::command]
pub fn toggle_main_window(app: tauri::AppHandle) {
    toggle_window(&app, "main");
}

/// 跳转到快捷键设置页面
#[tauri::command]
pub fn goto_shortcuts(app: tauri::AppHandle) {
    if let Some(window) = show_window(&app, "main") {
        // 发送页面跳转事件
        let _ = window.emit("goto-shortcuts", ());
    }
}

/// 显示并聚焦窗口
pub fn show_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        // 显示窗口
        match window.is_minimized() {
            Ok(true) => {
                let _ = window.unminimize();
            }
            Ok(false) | Err(_) => {
                let _ = window.show();
            }
        }

        // 聚焦窗口
        let _ = window.set_focus();

        Some(window)
    } else {
        None
    }
}

/// 隐藏窗口
pub fn hide_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.hide();

        // macOS 下同时隐藏 Dock 图标
        #[cfg(target_os = "macos")]
        {
            let _ = app.set_dock_visibility(false);
        }

        Some(window)
    } else {
        None
    }
}

/// 切换窗口显示状态
pub fn toggle_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        // 检查窗口可见性
        match window.is_visible() {
            Ok(true) => {
                match window.is_focused() {
                    Ok(true) => {
                        // 窗口可见且聚焦
                        hide_window(app, label);
                    }
                    Ok(false) | Err(_) => {
                        // 窗口可见但未聚焦
                        let _ = window.set_focus();
                    }
                }
            }
            Ok(false) | Err(_) => {
                // 窗口不可见
                show_window(app, label);
            }
        }

        Some(window)
    } else {
        None
    }
}
