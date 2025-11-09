use crate::error::AppError;
use crate::platform;
use crate::ENIGO;
use clipboard_rs::{Clipboard, ClipboardContext};
use enigo::{Direction, Key, Keyboard};
use log::warn;
use std::time::Duration;
use tokio::time::sleep;

/// Get selected text
#[tauri::command]
pub async fn get_selection(app: tauri::AppHandle) -> Result<String, AppError> {
    // try using platform native API to get selected text first
    if let Ok(text) = platform::get_selection() {
        if !text.is_empty() {
            return Ok(text);
        }
    }

    // if native API fails, fall back to clipboard method
    warn!("Failed to get selection natively, fallback to clipboard method");
    get_selection_fallback(app).await
}

/// Get selected text through clipboard
async fn get_selection_fallback(app: tauri::AppHandle) -> Result<String, AppError> {
    // create clipboard context
    let clipboard = ClipboardContext::new()
        .map_err(|e| format!("Failed to create clipboard context: {}", e))?;

    // use backup-operation-restore mode
    let selected_text = with_clipboard_backup(&clipboard, || async {
        // clear clipboard
        let _ = clipboard.clear();

        // send copy shortcut
        // https://github.com/enigo-rs/enigo/issues/153
        let _ = app.run_on_main_thread(move || {
            let _ = send_copy_keys();
        });

        // wait for clipboard content to change in a loop
        let max_wait_time = Duration::from_millis(200); // max wait time 200ms
        let check_interval = Duration::from_millis(5); // check interval 5ms
        let max_attempts = max_wait_time.as_millis() / check_interval.as_millis();

        let mut selected_text = String::new();

        for _attempt in 0..max_attempts {
            sleep(check_interval).await;

            // read current clipboard text
            if let Ok(current_text) = clipboard.get_text() {
                // if clipboard content changed, copy operation completed
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

/// send copy shortcut key
fn send_copy_keys() -> Result<(), AppError> {
    let mut enigo_guard = ENIGO.lock()?;
    let enigo = enigo_guard.as_mut()?;

    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;

    // release shift key
    enigo.key(Key::Shift, Direction::Release)?;

    // send Cmd+C or Ctrl+C
    enigo.key(modifier, Direction::Press)?;
    enigo.key(Key::Unicode('c'), Direction::Click)?;
    enigo.key(modifier, Direction::Release)?;

    Ok(())
}

/// backup clipboard contents, execute operation, then restore clipboard contents
async fn with_clipboard_backup<F, Fut, T>(clipboard: &ClipboardContext, operation: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    use clipboard_rs::ContentFormat;

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
    let result = operation().await;

    // restore original clipboard contents
    if !contents.is_empty() {
        if let Err(e) = clipboard.set(contents) {
            warn!("Failed to restore clipboard contents: {}", e);
        }
    }

    result
}
