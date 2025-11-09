use crate::error::AppError;
use clipboard_rs::{Clipboard, ClipboardContext};

/// get clipboard text content
#[tauri::command]
pub fn get_clipboard_text() -> Result<String, AppError> {
    Ok(ClipboardContext::new()
        .and_then(|c| c.get_text())
        .unwrap_or_default())
}
