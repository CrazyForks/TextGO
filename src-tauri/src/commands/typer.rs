use crate::error::AppError;
use crate::platform;
use crate::ENIGO;
use enigo::{Direction, Key, Keyboard};

#[tauri::command]
pub fn enter_text(text: String) -> Result<(), AppError> {
    if text.is_empty() {
        return Ok(());
    }

    // 使用 Enigo 输入文本
    let mut enigo_guard = ENIGO.lock()?;
    let enigo = enigo_guard.as_mut()?;
    enigo.text(&text)?;

    // 如果光标处可编辑，则尝试选中刚才输入的文本
    if platform::is_cursor_editable()? {
        // 延迟 100 毫秒，确保输入操作完成
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 计算需要选中的字符数
        let chars = text.chars().count();

        // 首先尝试使用原生 API 选中输出的文本
        if platform::select_backward_chars(chars).is_err() {
            // 原生 API 调用失败时，若字符数小于等于50，则使用键盘操作模拟
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
