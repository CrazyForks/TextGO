use crate::error::AppError;
use crate::REGISTERED_SHORTCUTS;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

/// register global shortcut CmdOrCtrl+Shift+<key>
#[tauri::command]
pub fn register_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
    // validate input parameters
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // check if already registered
    {
        let registered = REGISTERED_SHORTCUTS.lock()?;
        if registered.contains_key(&shortcut_str) {
            return Err(format!("Shortcut {} is already registered", shortcut_str).into());
        }
    }

    // create shortcut object
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

    let code_str = if key_upper.chars().all(|c| c.is_alphabetic()) {
        format!("Key{}", key_upper)
    } else {
        format!("Digit{}", key_upper)
    };

    let code = code_str.parse::<Code>().map_err(|_| "Unsupported key")?;
    let shortcut = Shortcut::new(Some(modifiers), code);

    // use plugin to register shortcut
    app.global_shortcut().register(shortcut)?;

    // save to registry
    {
        let mut registered = REGISTERED_SHORTCUTS.lock()?;
        registered.insert(shortcut_str.clone(), key_upper);
    }

    Ok(())
}

/// unregister global shortcut CmdOrCtrl+Shift+<key>
#[tauri::command]
pub fn unregister_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
    // validate input parameters
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // check if registered
    {
        let registered = REGISTERED_SHORTCUTS.lock()?;
        if !registered.contains_key(&shortcut_str) {
            return Err(format!("Shortcut {} is not registered", shortcut_str).into());
        }
    }

    // create shortcut object
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META | Modifiers::SHIFT;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

    let code_str = if key_upper.chars().all(|c| c.is_alphabetic()) {
        format!("Key{}", key_upper)
    } else {
        format!("Digit{}", key_upper)
    };

    let code = code_str.parse::<Code>().map_err(|_| "Unsupported key")?;
    let shortcut = Shortcut::new(Some(modifiers), code);

    // unregister shortcut
    app.global_shortcut().unregister(shortcut)?;

    // remove from registry
    {
        let mut registered = REGISTERED_SHORTCUTS.lock()?;
        registered.remove(&shortcut_str);
    }

    Ok(())
}

/// check if global shortcut CmdOrCtrl+Shift+<key> is registered
#[tauri::command]
pub fn is_shortcut_registered(key: String) -> Result<bool, AppError> {
    // validate input parameters
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("Shortcut key must be a single letter or digit".into());
    }

    let key_upper = key.to_uppercase();
    let shortcut_str = format!("CmdOrCtrl+Shift+{}", key_upper);

    // check registration status
    let registered = REGISTERED_SHORTCUTS.lock()?;
    let is_registered = registered.contains_key(&shortcut_str);

    Ok(is_registered)
}
