use crate::error::AppError;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;

/// 初始化或更新托盘菜单
#[tauri::command]
pub fn setup_tray(
    app: tauri::AppHandle,
    main_window_text: String,
    shortcuts_text: String,
    about_text: String,
    quit_text: String,
) -> Result<(), AppError> {
    // 创建新的菜单
    let menu = Menu::with_items(
        &app,
        &[
            &MenuItem::with_id(&app, "main_window", main_window_text, true, None::<&str>)?,
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

    // 尝试获取现有托盘菜单并更新
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(menu))?;
    } else {
        // 获取失败则创建新的托盘菜单
        let _tray = TrayIconBuilder::with_id("main-tray")
            .menu(&menu)
            .icon(app.default_window_icon().unwrap().clone())
            .icon_as_template(true)
            .show_menu_on_left_click(true)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "main_window" => {
                    crate::commands::toggle_main_window(app.clone());
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
            .build(&app)?;
    }

    Ok(())
}

/// 显示关于对话框
#[tauri::command]
pub fn show_about(app: tauri::AppHandle) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

    // 获取应用程序信息
    let package_info = app.package_info();
    // 使用 dialog 插件显示消息框
    app.dialog()
        .message(format!("Version {}", package_info.version))
        .title(package_info.name.clone())
        .kind(MessageDialogKind::Info)
        .blocking_show();
}
