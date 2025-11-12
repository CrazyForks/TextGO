use crate::error::AppError;
use crate::platform;
use crate::ENIGO;
use enigo::{Direction, Key, Keyboard};

/// Enter text and try to select it.
#[tauri::command]
pub fn enter_text(text: String) -> Result<(), AppError> {
    if text.is_empty() {
        return Ok(());
    }

    // use Enigo to enter text
    let mut enigo_guard = ENIGO.lock()?;
    let enigo = enigo_guard.as_mut()?;
    enigo.text(&text)?;

    // if cursor position is editable, try to select entered text
    if platform::is_cursor_editable()? {
        // delay 100 ms to ensure input operation completes
        std::thread::sleep(std::time::Duration::from_millis(100));

        // calculate number of characters to select
        let chars = text.chars().count();

        // first try using native API to select text
        if platform::select_backward_chars(chars).is_err() {
            // if native API call fails and char count is <= 50, use keyboard simulation
            if chars <= 50 {
                enigo.key(Key::Shift, Direction::Press)?;
                for _ in 0..chars {
                    enigo.key(Key::LeftArrow, Direction::Click)?;
                }
                enigo.key(Key::Shift, Direction::Release)?;
            }
        }
    }

    Ok(())
}
