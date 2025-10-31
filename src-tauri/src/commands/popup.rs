use crate::error::AppError;
use crate::ENIGO;
use enigo::Mouse;
use tauri::{Emitter, Manager};

#[tauri::command]
pub async fn show_popup(app: tauri::AppHandle, payload: String) -> Result<(), AppError> {
    // 获取当前鼠标位置
    let (mouse_x, mouse_y) = {
        let enigo = ENIGO
            .lock()
            .map_err(|e| format!("Failed to lock Enigo: {}", e))?;

        enigo
            .location()
            .map_err(|e| format!("Failed to get cursor position: {}", e))?
    };

    // 检查是否已有结果窗口
    if let Some(window) = app.get_webview_window("popup") {
        // 获取主显示器信息
        let monitor = window
            .current_monitor()
            .map_err(|e| format!("Failed to get current monitor: {}", e))?
            .ok_or_else(|| AppError::from("No monitor found"))?;

        // 获取缩放因子以统一单位
        let scale_factor = monitor.scale_factor();
        let screen_size = monitor.size();
        let screen_position = monitor.position();

        // 将物理像素转换为逻辑像素（与鼠标位置单位一致）
        let logical_screen_width = (screen_size.width as f64 / scale_factor) as i32;
        let logical_screen_height = (screen_size.height as f64 / scale_factor) as i32;
        let logical_screen_x = (screen_position.x as f64 / scale_factor) as i32;
        let logical_screen_y = (screen_position.y as f64 / scale_factor) as i32;

        // 使用配置文件中定义的逻辑窗口大小
        let window_width = 500; // 对应配置文件中的 width
        let window_height = 400; // 对应配置文件中的 height

        // 计算逻辑屏幕边界
        let screen_right = logical_screen_x + logical_screen_width;
        let screen_bottom = logical_screen_y + logical_screen_height;

        // 计算窗口应该在的位置，确保完全在屏幕内
        let margin = 20;
        let min_x = logical_screen_x + margin;
        let max_x = screen_right - window_width - margin;
        let min_y = logical_screen_y + margin;
        let max_y = screen_bottom - window_height - margin;

        // 优先尝试鼠标右下方
        let mut window_x = mouse_x + 15;
        let mut window_y = mouse_y + 15;

        // 限制在安全范围内
        window_x = window_x.clamp(min_x, max_x);
        window_y = window_y.clamp(min_y, max_y);

        // 设置调整后的窗口位置
        window
            .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                x: window_x as f64,
                y: window_y as f64,
            }))
            .map_err(|e| format!("Failed to set window position: {}", e))?;

        // 显示窗口
        window
            .show()
            .map_err(|e| format!("Failed to show window: {}", e))?;

        window
            .set_focus()
            .map_err(|e| format!("Failed to focus window: {}", e))?;

        // 发送数据到前端
        window
            .emit("popup", payload)
            .map_err(|e| format!("Failed to emit payload data: {}", e))?;
    } else {
        return Err("Popup window not found".into());
    }

    Ok(())
}
