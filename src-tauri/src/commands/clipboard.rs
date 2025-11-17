use crate::error::AppError;
use clipboard_rs::{Clipboard, ClipboardContext};
use log::warn;
use std::sync::Arc;

/// Get clipboard text content.
#[tauri::command]
pub fn get_clipboard_text() -> Result<String, AppError> {
    Ok(ClipboardContext::new()
        .and_then(|c| c.get_text())
        .unwrap_or_default())
}

/// Backup clipboard contents, execute operation, then restore clipboard contents.
pub async fn with_clipboard_backup<F, Fut, T>(operation: F) -> Result<T, AppError>
where
    F: FnOnce(Arc<ClipboardContext>) -> Fut,
    Fut: std::future::Future<Output = Result<T, AppError>>,
{
    use clipboard_rs::ContentFormat;

    // create clipboard context
    let clipboard = Arc::new(
        ClipboardContext::new()
            .map_err(|e| format!("Failed to create clipboard context: {}", e))?,
    );

    // backup all format contents
    let formats = [
        ContentFormat::Text,
        ContentFormat::Rtf,
        ContentFormat::Html,
        ContentFormat::Image,
        ContentFormat::Files,
    ];
    let contents = clipboard.get(&formats).unwrap_or_default();

    // execute operation
    let result = operation(clipboard.clone()).await?;

    // restore original clipboard contents
    if !contents.is_empty() {
        if let Err(e) = clipboard.set(contents) {
            warn!("Failed to restore clipboard contents: {}", e);
        }
    }

    Ok(result)
}
