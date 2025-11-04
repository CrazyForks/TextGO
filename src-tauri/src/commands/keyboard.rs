use crate::error::AppError;
use crate::ENIGO;
use enigo::{Direction, Key, Keyboard};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub fn send_copy_key() -> Result<(), AppError> {
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

#[tauri::command]
pub fn send_paste_key(app: tauri::AppHandle, text: String) -> Result<(), AppError> {
    // 计算字符数（在移动 text 之前）
    let char_count = text.chars().count();

    // 先将文本写入剪贴板
    let clipboard = app.clipboard();
    clipboard.write_text(text)?;

    let mut enigo_guard = ENIGO.lock()?;
    let enigo = enigo_guard.as_mut()?;

    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;

    // 释放 Shift 键
    enigo.key(Key::Shift, Direction::Release)?;

    // 发送 Cmd+V 或 Ctrl+V
    enigo.key(modifier, Direction::Press)?;
    enigo.key(Key::Unicode('v'), Direction::Click)?;
    enigo.key(modifier, Direction::Release)?;

    // 如果文字字符数小于等于50，模拟选中刚粘贴的文本
    if char_count <= 50 && char_count > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 按下 Shift 键
        enigo.key(Key::Shift, Direction::Press)?;

        // 按左方向键选中刚粘贴的文本
        for _ in 0..char_count {
            enigo.key(Key::LeftArrow, Direction::Click)?;
        }

        // 释放 Shift 键
        enigo.key(Key::Shift, Direction::Release)?;
    }

    Ok(())
}
