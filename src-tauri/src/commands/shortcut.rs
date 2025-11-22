use crate::error::AppError;
use crate::REGISTERED_SHORTCUTS;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

/// Register global shortcut.
#[tauri::command]
pub fn register_shortcut(app: tauri::AppHandle, shortcut: String) -> Result<(), AppError> {
    // check if registered
    if let Ok(registered) = is_shortcut_registered(shortcut.clone()) {
        if registered {
            return Err(format!("Shortcut {} is already registered", shortcut).into());
        }
    }

    // parse and create shortcut object
    let hotkey = parse_shortcut(&shortcut)?;

    // use plugin to register shortcut
    app.global_shortcut().register(hotkey)?;

    // save to registry
    {
        let mut registered = REGISTERED_SHORTCUTS.lock()?;
        registered.insert(hotkey.id, shortcut);
    }

    Ok(())
}

/// Unregister global shortcut.
#[tauri::command]
pub fn unregister_shortcut(app: tauri::AppHandle, shortcut: String) -> Result<(), AppError> {
    // check if registered
    if let Ok(registered) = is_shortcut_registered(shortcut.clone()) {
        if !registered {
            return Err(format!("Shortcut {} is not registered", shortcut).into());
        }
    }

    // parse and create shortcut object
    let hotkey = parse_shortcut(&shortcut)?;

    // unregister shortcut
    app.global_shortcut().unregister(hotkey)?;

    // remove from registry
    {
        let mut registered = REGISTERED_SHORTCUTS.lock()?;
        registered.remove(&hotkey.id);
    }

    Ok(())
}

/// Check if global shortcut is registered.
#[tauri::command]
pub fn is_shortcut_registered(shortcut: String) -> Result<bool, AppError> {
    // check registration status by checking values
    let registered = REGISTERED_SHORTCUTS.lock()?;
    let is_registered = registered.values().any(|v| v == &shortcut);
    Ok(is_registered)
}

/// Parse a shortcut string and create a Shortcut object.
/// Supported formats:
/// - "Meta+A", "Control+A", "Alt+A", "Shift+A"
/// - "Control+Shift+A", "Meta+Alt+A", etc.
fn parse_shortcut(shortcut: &str) -> Result<Shortcut, AppError> {
    // split by '+'
    let keys: Vec<&str> = shortcut.split('+').collect();
    if keys.is_empty() {
        return Err("Empty shortcut string".into());
    }

    // parse modifiers
    let mut modifiers = Modifiers::empty();
    for modifier in &keys[..keys.len() - 1] {
        match modifier.to_lowercase().as_str() {
            "meta" => modifiers |= Modifiers::META,
            "control" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            _ => return Err(format!("Unsupported modifier: {}", modifier).into()),
        }
    }

    // parse key code
    let code_str = keys.last().ok_or("Missing key code")?;
    let code = code_str
        .parse::<Code>()
        .map_err(|_| format!("Unsupported key code: {}", code_str))?;

    Ok(Shortcut::new(Some(modifiers), code))
}
