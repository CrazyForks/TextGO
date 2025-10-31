// 托盘菜单相关命令

use crate::error::AppError;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};

#[tauri::command]
pub async fn update_tray_menu(
    app: tauri::AppHandle,
    toggle_text: String,
    shortcuts_text: String,
    about_text: String,
    quit_text: String,
) -> Result<(), AppError> {
    // 获取托盘图标
    let tray = app.tray_by_id("main-tray").ok_or("Tray not found")?;

    // 创建新的菜单
    let menu = Menu::with_items(
        &app,
        &[
            &MenuItem::with_id(&app, "toggle", toggle_text, true, None::<&str>)?,
            &MenuItem::with_id(&app, "shortcuts", shortcuts_text, true, Some("CmdOrCtrl+,"))?,
            &PredefinedMenuItem::separator(&app)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::about(&app, Some(&about_text), None)?,
            #[cfg(not(target_os = "macos"))]
            &MenuItem::with_id(&app, "about", about_text, true, None::<&str>)?,
            &PredefinedMenuItem::separator(&app)?,
            &MenuItem::with_id(&app, "quit", quit_text, true, Some("CmdOrCtrl+Q"))?,
        ],
    )?;

    // 更新托盘菜单
    tray.set_menu(Some(menu))?;

    Ok(())
}

#[tauri::command]
pub fn show_about(app: tauri::AppHandle) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

    let package_info = app.package_info();

    // 使用 dialog 插件显示消息框
    app.dialog()
        .message(format!("Version {}", package_info.version))
        .title(package_info.name.clone())
        .kind(MessageDialogKind::Info)
        .blocking_show();
}
