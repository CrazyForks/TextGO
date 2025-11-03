use enigo::{Enigo, Settings};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tauri::{Emitter, Manager, RunEvent, WindowEvent};

// 模块导入
mod commands;
mod error;

use commands::*;

// Global, shared Enigo wrapped in a Mutex
// The Enigo struct should be created once and then reused for efficiency
pub static ENIGO: LazyLock<Mutex<Enigo>> =
    LazyLock::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

// Global registered shortcuts mapping
pub static REGISTERED_SHORTCUTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 全局快捷键处理函数
fn handle_global_shortcut(
    app: &tauri::AppHandle,
    shortcut: &tauri_plugin_global_shortcut::Shortcut,
    event: tauri_plugin_global_shortcut::ShortcutEvent,
) {
    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
        // 从 shortcut.key 格式化字符串中提取最后一个字符
        let key_str = format!("{}", shortcut.key);
        let key_char = key_str.chars().last().unwrap_or('?').to_string();

        let app_clone = app.clone();
        let key_char_clone = key_char.clone();

        // 异步获取选中文本并发送事件到前端
        tauri::async_runtime::spawn(async move {
            match get_selection(app_clone.clone()).await {
                Ok(selection) => {
                    let event_data = serde_json::json!({
                        "key": key_char_clone,
                        "selection": selection
                    });
                    if let Err(e) = app_clone.emit("shortcut-triggered", event_data) {
                        eprintln!("[ERROR] Failed to emit shortcut event: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("[ERROR] Failed to get selection: {}", e);
                }
            }
        });
    }
}

/// 应用设置函数
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle().clone();

    // 初始化托盘菜单
    setup_tray(
        app_handle.clone(),
        "Show / Hide".to_string(),
        "Edit Shortcuts...".to_string(),
        "About TextGO".to_string(),
        "Quit".to_string(),
    )
    .ok();

    // 获取主窗口
    if let Some(window) = app.get_webview_window("main") {
        let app_handle = window.app_handle().clone();

        // 由于窗口初始时是隐藏的，隐藏 Dock 图标
        #[cfg(target_os = "macos")]
        {
            let _ = app_handle.set_dock_visibility(false);
        }

        // 设置窗口关闭行为，关闭时隐藏而不是退出
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 阻止默认关闭行为
                api.prevent_close();
                // 隐藏窗口到系统托盘
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.hide();
                    // 隐藏窗口时，隐藏 Dock 图标
                    #[cfg(target_os = "macos")]
                    {
                        let _ = app_handle.set_dock_visibility(false);
                    }
                }
            }
        });
    }

    // 获取 popup 窗口并设置关闭行为
    if let Some(window) = app.get_webview_window("popup") {
        let app_handle = window.app_handle().clone();
        // 设置 popup 窗口关闭行为，关闭时隐藏而不是销毁
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 阻止默认关闭行为
                api.prevent_close();
                // 隐藏窗口而不是销毁
                if let Some(popup) = app_handle.get_webview_window("popup") {
                    let _ = popup.hide();
                }
            }
        });
    }

    Ok(())
}

/// 运行时事件处理函数
fn handle_run_event(app: &tauri::AppHandle, #[allow(unused_variables)] event: RunEvent) {
    // 处理 macOS 下的 Reopen 事件
    #[cfg(target_os = "macos")]
    if let RunEvent::Reopen {
        has_visible_windows: false,
        ..
    } = event
    {
        // 没有可见窗口时，显示主窗口
        show_window(app, "main");
        // 同时显示 Dock 图标
        let _ = app.set_dock_visibility(true);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(handle_global_shortcut)
                .build(),
        )
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            hide_main_window,
            toggle_main_window,
            goto_shortcuts,
            send_copy_key,
            send_paste_key,
            get_selection,
            is_editable,
            show_popup,
            execute_javascript,
            execute_python,
            register_shortcut,
            unregister_shortcut,
            is_shortcut_registered,
            setup_tray,
            show_about
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(handle_run_event);
}
