#[derive(Debug, Clone)]
pub struct AppError(String);

impl std::error::Error for AppError {}

// 实现 Display，使错误信息可以直接打印
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 实现 Serialize，使其可以作为 Tauri 命令的返回类型
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::from(error.as_str())
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        eprintln!("[ERROR] {}", error);
        AppError(error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        eprintln!("[ERROR] IO error: {}", error);
        AppError(error.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        eprintln!("[ERROR] Mutex lock poisoned: {}", error);
        AppError(error.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        eprintln!("[ERROR] Tauri error: {}", error);
        AppError(error.to_string())
    }
}

impl From<tauri_plugin_clipboard_manager::Error> for AppError {
    fn from(error: tauri_plugin_clipboard_manager::Error) -> Self {
        eprintln!("[ERROR] Clipboard error: {}", error);
        AppError(error.to_string())
    }
}

impl From<tauri_plugin_global_shortcut::Error> for AppError {
    fn from(error: tauri_plugin_global_shortcut::Error) -> Self {
        eprintln!("[ERROR] Global shortcut error: {}", error);
        AppError(error.to_string())
    }
}

impl From<enigo::InputError> for AppError {
    fn from(error: enigo::InputError) -> Self {
        eprintln!("[ERROR] Enigo input error: {}", error);
        AppError(error.to_string())
    }
}

impl From<&mut enigo::NewConError> for AppError {
    fn from(error: &mut enigo::NewConError) -> Self {
        AppError::from(&*error)
    }
}

impl From<&enigo::NewConError> for AppError {
    fn from(error: &enigo::NewConError) -> Self {
        eprintln!("[ERROR] Enigo initialization error: {}", error);
        AppError(error.to_string())
    }
}
