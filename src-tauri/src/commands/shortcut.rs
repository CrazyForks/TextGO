// 快捷键管理相关命令

use crate::error::AppError;
use crate::REGISTERED_SHORTCUTS;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[tauri::command]
pub async fn register_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
    // 验证输入参数
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // 检查是否已经注册
    {
        let registered = REGISTERED_SHORTCUTS
            .lock()
            .map_err(|e| format!("Failed to lock: {}", e))?;
        if registered.contains_key(&shortcut_str) {
            return Err(format!("Shortcut {} is already registered", shortcut_str).into());
        }
    }

    // 创建快捷键对象
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

    let code_str = if key_upper.chars().all(|c| c.is_alphabetic()) {
        format!("Key{}", key_upper)
    } else {
        format!("Digit{}", key_upper)
    };

    let code = code_str
        .parse::<Code>()
        .map_err(|_| AppError::from("Unsupported key"))?;
    let shortcut = Shortcut::new(Some(modifiers), code);

    // 使用插件注册快捷键
    app.global_shortcut()
        .register(shortcut)
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    // 保存到注册表
    {
        let mut registered = REGISTERED_SHORTCUTS
            .lock()
            .map_err(|e| format!("Failed to lock: {}", e))?;
        registered.insert(shortcut_str.clone(), key_upper);
    }

    Ok(())
}

#[tauri::command]
pub async fn unregister_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
    // 验证输入参数
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // 检查是否已注册
    {
        let registered = REGISTERED_SHORTCUTS
            .lock()
            .map_err(|e| format!("Failed to lock: {}", e))?;
        if !registered.contains_key(&shortcut_str) {
            return Err(format!("Shortcut {} is not registered", shortcut_str).into());
        }
    }

    // 创建快捷键对象
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

    let code_str = if key_upper.chars().all(|c| c.is_alphabetic()) {
        format!("Key{}", key_upper)
    } else {
        format!("Digit{}", key_upper)
    };

    let code = code_str
        .parse::<Code>()
        .map_err(|_| AppError::from("Unsupported key"))?;
    let shortcut = Shortcut::new(Some(modifiers), code);

    // 注销快捷键
    app.global_shortcut()
        .unregister(shortcut)
        .map_err(|e| format!("Failed to unregister shortcut: {}", e))?;

    // 从注册表中移除
    {
        let mut registered = REGISTERED_SHORTCUTS
            .lock()
            .map_err(|e| format!("Failed to lock: {}", e))?;
        registered.remove(&shortcut_str);
    }

    Ok(())
}

#[tauri::command]
pub async fn is_shortcut_registered(key: String) -> Result<bool, AppError> {
    // 验证输入参数
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // 检查注册状态
    let registered = REGISTERED_SHORTCUTS
        .lock()
        .map_err(|e| format!("Failed to lock: {}", e))?;
    let is_registered = registered.contains_key(&shortcut_str);

    Ok(is_registered)
}
