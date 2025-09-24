use crate::error::AppError;
use clipboard_rs::{Clipboard, ClipboardContext};

/// 获取剪贴板文本内容
#[tauri::command]
pub fn get_clipboard_text() -> Result<String, AppError> {
    Ok(ClipboardContext::new()
        .and_then(|c| c.get_text())
        .unwrap_or_default())
}
