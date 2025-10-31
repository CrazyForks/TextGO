// 自定义错误类型

#[derive(Debug, Clone)]
pub struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        eprintln!("[ERROR] {}", error);
        AppError(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        eprintln!("[ERROR] {}", error);
        AppError(error.to_string())
    }
}

impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        eprintln!("[ERROR] {}", error);
        AppError(error.to_string())
    }
}

// 实现 Serialize，使其可以作为 Tauri 命令的返回类型
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
