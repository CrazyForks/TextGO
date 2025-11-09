mod commands;
mod error;
mod platform;

use commands::*;
use enigo::{Enigo, Settings};
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tauri::{Emitter, Manager, RunEvent, WindowEvent};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutEvent, ShortcutState};
use tauri_plugin_log::{Target, TargetKind};

// Global, shared Enigo wrapped in a Mutex
// The Enigo struct should be created once and then reused for efficiency
pub static ENIGO: LazyLock<Mutex<Result<Enigo, enigo::NewConError>>> =
    LazyLock::new(|| Mutex::new(Enigo::new(&Settings::default())));

// Global registered shortcuts mapping
pub static REGISTERED_SHORTCUTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Application setup function
fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle().clone();

    // initialize tray menu
    setup_tray(
        app_handle.clone(),
        "Show / Hide".to_string(),
        "Edit Shortcuts...".to_string(),
        "About TextGO".to_string(),
        "Quit".to_string(),
    )
    .ok();

    // get main window
    if let Some(window) = app.get_webview_window("main") {
        let app_handle = window.app_handle().clone();

        // since window is initially hidden, hide dock icon
        #[cfg(target_os = "macos")]
        let _ = app_handle.set_dock_visibility(false);

        // set window close behavior, hide instead of quit on close
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // prevent default close behavior
                api.prevent_close();
                // hide window to system tray
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.hide();
                    // when hiding window, also hide dock icon
                    #[cfg(target_os = "macos")]
                    let _ = app_handle.set_dock_visibility(false);
                }
            }
        });
    }

    // get popup window and set close behavior
    if let Some(window) = app.get_webview_window("popup") {
        let app_handle = window.app_handle().clone();
        // set popup window close behavior, hide instead of destroy on close
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // prevent default close behavior
                api.prevent_close();
                // hide window instead of destroy
                if let Some(popup) = app_handle.get_webview_window("popup") {
                    let _ = popup.hide();
                }
            }
        });
    }

    Ok(())
}

/// Runtime event handler function
#[allow(unused_variables)]
fn handle_run_event(app: &tauri::AppHandle, event: RunEvent) {
    // handle Reopen event on macOS
    #[cfg(target_os = "macos")]
    if let RunEvent::Reopen {
        has_visible_windows: false,
        ..
    } = event
    {
        // show main window when no visible windows
        show_window(app, "main");
        // also show dock icon
        let _ = app.set_dock_visibility(true);
    }
}

/// Global shortcut handler function
fn handle_shortcut_event(app: &tauri::AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() == ShortcutState::Pressed {
        // extract last character from shortcut.key formatted string
        let key_str = format!("{}", shortcut.key);
        let key_char = key_str.chars().last().unwrap_or('?').to_string();

        let app_clone = app.clone();
        let key_char_clone = key_char.clone();

        // asynchronously get selected text and emit event to frontend
        tauri::async_runtime::spawn(async move {
            if let Ok(selection) = get_selection(app_clone.clone()).await {
                let event_data = serde_json::json!({
                    "key": key_char_clone,
                    "selection": selection
                });
                let _ = app_clone.emit("shortcut-triggered", event_data);
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(handle_shortcut_event)
                .build(),
        )
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(Target::new(TargetKind::Stdout))
                .with_colors(ColoredLevelConfig::default())
                .level(if cfg!(dev) {
                    LevelFilter::Info
                } else {
                    LevelFilter::Off
                })
                .build(),
        )
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            hide_main_window,
            toggle_main_window,
            goto_shortcuts,
            register_shortcut,
            unregister_shortcut,
            is_shortcut_registered,
            get_selection,
            execute_python,
            execute_javascript,
            get_clipboard_text,
            enter_text,
            show_popup,
            show_about,
            setup_tray
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(handle_run_event);
}
