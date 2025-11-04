use crate::commands::send_copy_key;
use crate::error::AppError;
use std::time::Duration;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::time::sleep;

#[tauri::command]
pub async fn get_selection(app: tauri::AppHandle) -> Result<String, AppError> {
    // 获取剪贴板管理器
    let clipboard = app.clipboard();

    // 保存当前剪贴板内容
    let original_clipboard = clipboard.read_text().unwrap_or_else(|_| String::new());

    // 清空剪贴板内容
    clipboard.clear()?;

    // 发送复制快捷键
    // https://github.com/enigo-rs/enigo/issues/153
    let _ = app.run_on_main_thread(move || {
        let _ = send_copy_key();
    });

    // 循环等待剪贴板内容变化
    let max_wait_time = Duration::from_millis(500); // 最大等待时间 500ms
    let check_interval = Duration::from_millis(10); // 每次检查间隔 10ms
    let max_attempts = max_wait_time.as_millis() / check_interval.as_millis();

    for _attempt in 0..max_attempts {
        sleep(check_interval).await;

        // 读取当前剪贴板内容
        if let Ok(current_clipboard) = clipboard.read_text() {
            // 如果剪贴板内容发生了变化，说明复制操作完成
            if !current_clipboard.is_empty() {
                // 恢复原来的剪贴板内容（如果有的话）
                if !original_clipboard.is_empty() {
                    clipboard.write_text(original_clipboard)?;
                }
                return Ok(current_clipboard);
            }
        }
    }

    // 超时后仍然没有变化，可能没有选中任何文字
    let warning_msg = format!(
        "Clipboard content did not change within {}ms, possibly no text selected",
        max_wait_time.as_millis()
    );
    eprintln!("[WARNING] {}", warning_msg);

    // 恢复原来的剪贴板内容
    if !original_clipboard.is_empty() {
        clipboard.write_text(original_clipboard.clone())?;
    }

    Ok(String::new())
}
