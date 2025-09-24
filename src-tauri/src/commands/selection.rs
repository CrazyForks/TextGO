use crate::error::AppError;
use crate::platform;
use crate::ENIGO;
use clipboard_rs::{Clipboard, ClipboardContext};
use enigo::{Direction, Key, Keyboard};
use log::warn;
use std::time::Duration;
use tokio::time::sleep;

#[tauri::command]
pub async fn get_selection(app: tauri::AppHandle) -> Result<String, AppError> {
    // 优先尝试使用平台原生 API 获取选中文本
    if let Ok(text) = platform::get_selection() {
        if !text.is_empty() {
            return Ok(text);
        }
    }

    // 原生 API 获取失败，降级使用剪贴板方案
    warn!("Failed to get selection natively, fallback to clipboard method");
    get_selection_fallback(app).await
}

/// 通过剪贴板获取选中文本
async fn get_selection_fallback(app: tauri::AppHandle) -> Result<String, AppError> {
    // 创建剪贴板上下文
    let clipboard = ClipboardContext::new()
        .map_err(|e| format!("Failed to create clipboard context: {}", e))?;

    // 使用备份-操作-恢复模式
    let selected_text = with_clipboard_backup(&clipboard, || async {
        // 清空剪贴板
        let _ = clipboard.clear();

        // 发送复制快捷键
        // https://github.com/enigo-rs/enigo/issues/153
        let _ = app.run_on_main_thread(move || {
            let _ = send_copy_keys();
        });

        // 循环等待剪贴板内容变化
        let max_wait_time = Duration::from_millis(200); // 最大等待时间 200ms
        let check_interval = Duration::from_millis(5); // 每次检查间隔 5ms
        let max_attempts = max_wait_time.as_millis() / check_interval.as_millis();

        let mut selected_text = String::new();

        for _attempt in 0..max_attempts {
            sleep(check_interval).await;

            // 读取当前剪贴板文本内容
            if let Ok(current_text) = clipboard.get_text() {
                // 如果剪贴板内容发生了变化，说明复制操作完成
                if !current_text.is_empty() {
                    selected_text = current_text;
                    break;
                }
            }
        }

        if selected_text.is_empty() {
            warn!(
                "Clipboard did not change within {} ms, possibly no text selected",
                max_wait_time.as_millis()
            );
        }

        selected_text
    })
    .await;

    Ok(selected_text)
}

fn send_copy_keys() -> Result<(), AppError> {
    let mut enigo_guard = ENIGO.lock()?;
    let enigo = enigo_guard.as_mut()?;

    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;

    // 释放 Shift 键
    enigo.key(Key::Shift, Direction::Release)?;

    // 发送 Cmd+C 或 Ctrl+C
    enigo.key(modifier, Direction::Press)?;
    enigo.key(Key::Unicode('c'), Direction::Click)?;
    enigo.key(modifier, Direction::Release)?;

    Ok(())
}

/// 备份剪贴板内容，执行操作，然后恢复剪贴板内容
async fn with_clipboard_backup<F, Fut, T>(clipboard: &ClipboardContext, operation: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    use clipboard_rs::ContentFormat;

    // 备份所有格式内容
    let formats = [
        ContentFormat::Text,
        ContentFormat::Rtf,
        ContentFormat::Html,
        ContentFormat::Image,
        ContentFormat::Files,
    ];
    let contents = clipboard.get(&formats).unwrap_or_default();

    // 执行操作
    let result = operation().await;

    // 恢复原来的剪贴板内容
    if !contents.is_empty() {
        if let Err(e) = clipboard.set(contents) {
            warn!("Failed to restore clipboard contents: {}", e);
        }
    }

    result
}
