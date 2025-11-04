use crate::commands::window::show_window;
use crate::error::AppError;
use crate::ENIGO;
use enigo::Mouse;
use tauri::{Emitter, Manager};

// 弹窗窗口宽度
const POPUP_WINDOW_WIDTH: i32 = 400;
// 弹窗窗口高度
const POPUP_WINDOW_HEIGHT: i32 = 300;
// 弹窗窗口偏移量
const POPUP_MOUSE_OFFSET: i32 = 10;
// 底部安全区域高度
const POPUP_SAFE_AREA_BOTTOM: i32 = 80;

/// 显示弹窗并定位到鼠标附近
#[tauri::command]
pub fn show_popup(app: tauri::AppHandle, payload: String) -> Result<(), AppError> {
    // 获取当前鼠标位置
    let (mouse_x, mouse_y) = {
        let enigo_lock = ENIGO.lock()?;
        let enigo = enigo_lock.as_ref()?;
        enigo.location()?
    };

    // 检查是否已有结果窗口
    if let Some(window) = app.get_webview_window("popup") {
        // 获取主显示器信息
        let monitor = window
            .current_monitor()?
            .ok_or_else(|| AppError::from("No monitor found"))?;
        let size = monitor.size();
        let position = monitor.position();
        let scale_factor = monitor.scale_factor();

        // 将物理像素转换为逻辑像素
        let screen_width = (size.width as f64 / scale_factor) as i32;
        let screen_height = (size.height as f64 / scale_factor) as i32;
        let screen_x = (position.x as f64 / scale_factor) as i32;
        let screen_y = (position.y as f64 / scale_factor) as i32;

        // 计算窗口的安全范围
        let min_x = screen_x;
        let max_x = screen_x + screen_width - POPUP_WINDOW_WIDTH;
        let min_y = screen_y;
        let max_y = screen_y + screen_height - POPUP_WINDOW_HEIGHT - POPUP_SAFE_AREA_BOTTOM;

        // 设置调整后的窗口位置
        window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: (mouse_x + POPUP_MOUSE_OFFSET).clamp(min_x, max_x) as f64,
            y: (mouse_y + POPUP_MOUSE_OFFSET).clamp(min_y, max_y) as f64,
        }))?;

        // 增加一定的延迟防止闪烁
        if !window.is_visible()? {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // 显示窗口
        show_window(&app, "popup");

        // 发送数据
        window.emit("popup", payload)?;
    } else {
        return Err("Popup window not found".into());
    }

    Ok(())
}
