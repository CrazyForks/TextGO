use crate::commands::window::show_window;
use crate::error::AppError;
use crate::ENIGO;
use enigo::Mouse;
use tauri::{Emitter, Manager};

// popup window width
const POPUP_WINDOW_WIDTH: i32 = 400;
// popup window height
const POPUP_WINDOW_HEIGHT: i32 = 300;
// popup window offset
const POPUP_WINDOW_OFFSET: i32 = 10;
// bottom safe area height
const POPUP_SAFE_AREA_BOTTOM: i32 = 80;

/// Show popup and position it near the mouse.
#[tauri::command]
pub fn show_popup(app: tauri::AppHandle, payload: String) -> Result<(), AppError> {
    // get current mouse position
    let (mouse_x, mouse_y) = {
        let enigo_guard = ENIGO.lock()?;
        let enigo = enigo_guard.as_ref()?;
        enigo.location()?
    };

    // check if result window already exists
    if let Some(window) = app.get_webview_window("popup") {
        // get main monitor info
        let monitor = window
            .current_monitor()?
            .ok_or_else(|| AppError::from("No monitor found"))?;
        let size = monitor.size();
        let position = monitor.position();
        let scale_factor = monitor.scale_factor();

        // convert physical pixels to logical pixels
        let screen_width = (size.width as f64 / scale_factor) as i32;
        let screen_height = (size.height as f64 / scale_factor) as i32;
        let screen_x = (position.x as f64 / scale_factor) as i32;
        let screen_y = (position.y as f64 / scale_factor) as i32;

        // calculate safe area for window
        let min_x = screen_x;
        let max_x = screen_x + screen_width - POPUP_WINDOW_WIDTH;
        let min_y = screen_y;
        let max_y = screen_y + screen_height - POPUP_WINDOW_HEIGHT - POPUP_SAFE_AREA_BOTTOM;

        // set adjusted window position
        window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: (mouse_x + POPUP_WINDOW_OFFSET).clamp(min_x, max_x) as f64,
            y: (mouse_y + POPUP_WINDOW_OFFSET).clamp(min_y, max_y) as f64,
        }))?;

        // add some delay to prevent flickering
        if !window.is_visible()? {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // show window
        show_window(&app, "popup");

        // send data
        window.emit("popup", payload)?;
    } else {
        return Err("Popup window not found".into());
    }

    Ok(())
}
