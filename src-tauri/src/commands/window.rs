use tauri::{Emitter, Manager, WebviewWindow};

/// show main window
#[tauri::command]
pub fn show_main_window(app: tauri::AppHandle) {
    show_window(&app, "main");
}

/// hide main window
#[tauri::command]
pub fn hide_main_window(app: tauri::AppHandle) {
    hide_window(&app, "main");
}

/// toggle main window visibility
#[tauri::command]
pub fn toggle_main_window(app: tauri::AppHandle) {
    toggle_window(&app, "main");
}

/// navigate to shortcut settings page
#[tauri::command]
pub fn goto_shortcuts(app: tauri::AppHandle) {
    if let Some(window) = show_window(&app, "main") {
        // emit page navigation event
        let _ = window.emit("goto-shortcuts", ());
    }
}

/// show and focus window
pub fn show_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        if window.is_minimized().unwrap_or(false) {
            // unminimize
            let _ = window.unminimize();
        } else {
            // show window
            let _ = window.show();
        }
        // focus window
        let _ = window.set_focus();

        Some(window)
    } else {
        None
    }
}

/// hide window
pub fn hide_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.hide();

        // also hide dock icon on macOS
        #[cfg(target_os = "macos")]
        let _ = app.set_dock_visibility(false);

        Some(window)
    } else {
        None
    }
}

/// toggle window visibility
pub fn toggle_window(app: &tauri::AppHandle, label: &str) -> Option<WebviewWindow> {
    if let Some(window) = app.get_webview_window(label) {
        // check if window is minimized
        if window.is_minimized().unwrap_or(false) {
            let _ = window.unminimize();
            return Some(window);
        }

        // check if window is not visible
        if !window.is_visible().unwrap_or(false) {
            return show_window(app, label);
        }

        // check if window is not focused
        #[cfg(target_os = "macos")]
        if !window.is_focused().unwrap_or(false) {
            return show_window(app, label);
        }

        // hide when window is visible and not minimized
        hide_window(app, label)
    } else {
        None
    }
}
