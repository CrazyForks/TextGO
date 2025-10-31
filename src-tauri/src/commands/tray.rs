// 托盘菜单相关命令

use crate::error::AppError;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;

/// 初始化或更新托盘菜单
#[tauri::command]
pub async fn setup_tray_menu(
    app: tauri::AppHandle,
    toggle_text: String,
    shortcuts_text: String,
    about_text: String,
    quit_text: String,
) -> Result<(), AppError> {
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
    )
    .map_err(|e| AppError::from(format!("Failed to create menu: {}", e)))?;

    // 尝试获取现有托盘图标并更新菜单
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(menu))
            .map_err(|e| AppError::from(format!("Failed to update tray menu: {}", e)))?;
    } else {
        // 如果托盘不存在，创建新的托盘图标
        let _tray = TrayIconBuilder::with_id("main-tray")
            .menu(&menu)
            .icon(app.default_window_icon().unwrap().clone())
            .icon_as_template(true)
            .show_menu_on_left_click(true)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "toggle" => {
                    crate::commands::toggle_window(app.clone());
                }
                "shortcuts" => {
                    crate::commands::goto_shortcuts(app.clone());
                }
                "about" => {
                    show_about(app.clone());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            })
            .build(&app)
            .map_err(|e| AppError::from(format!("Failed to build tray icon: {}", e)))?;
    }

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
