use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};
use serde_json;
use std::{
    collections::HashMap,
    process::Stdio,
    sync::{LazyLock, Mutex},
    time::Duration,
};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager, RunEvent, WindowEvent,
};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use tokio::{io::AsyncWriteExt, process::Command, time::sleep};

// 自定义错误类型，自动打印错误到控制台
#[derive(Debug, Clone)]
pub struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        // 自动打印错误
        eprintln!("[ERROR] {}", error);
        AppError(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        // 自动打印错误
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

// Global, shared Enigo wrapped in a Mutex
// The Enigo struct should be created once and then reused for efficiency
static ENIGO: LazyLock<Mutex<Enigo>> =
    LazyLock::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

// Global registered shortcuts mapping
static REGISTERED_SHORTCUTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn show_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
    }
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        // 隐藏窗口时，隐藏 Dock 图标
        #[cfg(target_os = "macos")]
        {
            let _ = app.set_dock_visibility(false);
        }
    }
}

#[tauri::command]
fn toggle_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                // 窗口可见，检查是否聚焦
                match window.is_focused() {
                    Ok(true) => {
                        // 窗口可见且聚焦，调用 hide_window
                        hide_window(app);
                    }
                    Ok(false) | Err(_) => {
                        // 窗口可见但未聚焦，把窗口置于前端并聚焦
                        let _ = window.set_focus();
                        let _ = window.unminimize();
                    }
                }
            }
            Ok(false) | Err(_) => {
                // 窗口隐藏或出错，调用 show_window
                show_window(app);
            }
        }
    }
}

#[tauri::command]
fn show_shortcuts(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
        // 发送页面跳转事件
        let _ = window.emit("goto-shortcuts", ());
    }
}

#[tauri::command]
fn send_copy_key() -> Result<(), AppError> {
    let mut enigo = ENIGO
        .lock()
        .map_err(|e| format!("Failed to lock Enigo: {}", e))?;

    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;

    // 发送 Cmd+C 或 Ctrl+C
    enigo
        .key(modifier, Direction::Press)
        .map_err(|e| e.to_string())?;
    enigo
        .key(Key::Unicode('c'), Direction::Click)
        .map_err(|e| e.to_string())?;
    enigo
        .key(modifier, Direction::Release)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn send_paste_key() -> Result<(), AppError> {
    let mut enigo = ENIGO
        .lock()
        .map_err(|e| format!("Failed to lock Enigo: {}", e))?;

    #[cfg(target_os = "macos")]
    let modifier = Key::Meta;
    #[cfg(not(target_os = "macos"))]
    let modifier = Key::Control;

    // 发送 Cmd+V 或 Ctrl+V
    enigo
        .key(modifier, Direction::Press)
        .map_err(|e| e.to_string())?;
    enigo
        .key(Key::Unicode('v'), Direction::Click)
        .map_err(|e| e.to_string())?;
    enigo
        .key(modifier, Direction::Release)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_selection(app: tauri::AppHandle) -> Result<String, AppError> {
    // 获取剪贴板管理器
    let clipboard = app.clipboard();

    // 保存当前剪贴板内容
    let original_clipboard = clipboard.read_text().unwrap_or_else(|_| String::new());

    // 清空剪贴板内容
    if let Err(e) = clipboard.clear() {
        return Err(format!("Failed to clear clipboard: {}", e).into());
    }

    // 发送复制快捷键
    // https://github.com/enigo-rs/enigo/issues/153
    let _ = app.run_on_main_thread(move || {
        let _ = send_copy_key();
    });

    // 循环等待剪贴板内容变化
    let max_wait_time = Duration::from_millis(500); // 最大等待时间 500ms
    let check_interval = Duration::from_millis(50); // 每次检查间隔 50ms
    let max_attempts = max_wait_time.as_millis() / check_interval.as_millis();

    for _attempt in 0..max_attempts {
        sleep(check_interval).await;

        // 读取当前剪贴板内容
        if let Ok(current_clipboard) = clipboard.read_text() {
            // 如果剪贴板内容发生了变化，说明复制操作完成
            if !current_clipboard.is_empty() {
                // 恢复原来的剪贴板内容（如果有的话）
                if !original_clipboard.is_empty() {
                    if let Err(e) = clipboard.write_text(original_clipboard) {
                        return Err(format!("Failed to restore clipboard content: {}", e).into());
                    }
                }
                return Ok(current_clipboard);
            }
        }
    }

    // 超时后仍然没有变化，可能没有选中任何文字
    let warning_msg = format!(
        "Clipboard content did not change within {}ms, possibly no text selected",
        max_wait_time.as_millis()
    );
    eprintln!("[WARNING] {}", warning_msg);

    // 恢复原来的剪贴板内容
    if !original_clipboard.is_empty() {
        if let Err(e) = clipboard.write_text(original_clipboard.clone()) {
            return Err(format!("Failed to restore clipboard content: {}", e).into());
        }
    }

    Ok(original_clipboard)
}

#[tauri::command]
async fn show_popup_window(app: tauri::AppHandle, payload: String) -> Result<(), AppError> {
    // 获取当前鼠标位置
    let (mouse_x, mouse_y) = {
        let enigo = ENIGO
            .lock()
            .map_err(|e| format!("Failed to lock Enigo: {}", e))?;

        enigo
            .location()
            .map_err(|e| format!("Failed to get cursor position: {}", e))?
    };

    // 检查是否已有结果窗口
    if let Some(window) = app.get_webview_window("popup") {
        // 获取主显示器信息
        let monitor = window
            .current_monitor()
            .map_err(|e| format!("Failed to get current monitor: {}", e))?
            .ok_or_else(|| AppError::from("No monitor found"))?;

        // 获取缩放因子以统一单位
        let scale_factor = monitor.scale_factor();
        let screen_size = monitor.size();
        let screen_position = monitor.position();

        // 将物理像素转换为逻辑像素（与鼠标位置单位一致）
        let logical_screen_width = (screen_size.width as f64 / scale_factor) as i32;
        let logical_screen_height = (screen_size.height as f64 / scale_factor) as i32;
        let logical_screen_x = (screen_position.x as f64 / scale_factor) as i32;
        let logical_screen_y = (screen_position.y as f64 / scale_factor) as i32;

        // 使用配置文件中定义的逻辑窗口大小
        let window_width = 500; // 对应配置文件中的 width
        let window_height = 400; // 对应配置文件中的 height

        // 计算逻辑屏幕边界
        let screen_right = logical_screen_x + logical_screen_width;
        let screen_bottom = logical_screen_y + logical_screen_height;

        // 计算窗口应该在的位置，确保完全在屏幕内
        let margin = 20;
        let min_x = logical_screen_x + margin;
        let max_x = screen_right - window_width - margin;
        let min_y = logical_screen_y + margin;
        let max_y = screen_bottom - window_height - margin;

        // 优先尝试鼠标右下方
        let mut window_x = mouse_x + 15;
        let mut window_y = mouse_y + 15;

        // 限制在安全范围内
        window_x = window_x.clamp(min_x, max_x);
        window_y = window_y.clamp(min_y, max_y);

        // 设置调整后的窗口位置
        window
            .set_position(tauri::Position::Logical(tauri::LogicalPosition {
                x: window_x as f64,
                y: window_y as f64,
            }))
            .map_err(|e| format!("Failed to set window position: {}", e))?;

        // 显示窗口
        window
            .show()
            .map_err(|e| format!("Failed to show window: {}", e))?;

        window
            .set_focus()
            .map_err(|e| format!("Failed to focus window: {}", e))?;

        // 发送数据到前端
        window
            .emit("popup", payload)
            .map_err(|e| format!("Failed to emit payload data: {}", e))?;
    } else {
        return Err("Popup window not found".into());
    }

    Ok(())
}

#[tauri::command]
async fn execute_javascript(
    code: String,
    data: String,
    node_path: Option<String>,
) -> Result<String, AppError> {
    // 创建 JavaScript 代码包装
    let wrapped_code = format!(
        r#"
const data = {};
{}
const result = process(data);
console.log(typeof result === 'string' ? result : JSON.stringify(result));
        "#,
        data, code
    );

    // 如果提供了自定义路径，直接使用
    if let Some(program) = node_path {
        if !program.is_empty() {
            // 在 Windows 上，如果是 .bat 文件，需要特殊处理
            #[cfg(target_os = "windows")]
            let use_stdin = program.to_lowercase().ends_with(".bat");
            #[cfg(not(target_os = "windows"))]
            let use_stdin = false;

            let mut command = if use_stdin {
                // 对于 .bat 文件，通过 stdin 传递代码，避免参数转义问题
                let mut cmd = Command::new(&program);
                cmd.stdin(Stdio::piped());
                cmd
            } else {
                // 对于普通可执行文件，使用 -e 参数
                let mut cmd = Command::new(&program);
                cmd.arg("-e").arg(&wrapped_code);
                cmd
            };

            command
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            match command.spawn() {
                Ok(mut child) => {
                    // 如果使用 stdin，写入代码
                    if use_stdin {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(wrapped_code.as_bytes())
                                .await
                                .map_err(|e| format!("Failed to write to stdin: {}", e))?;
                            drop(stdin); // 关闭 stdin
                        }
                    }

                    let output = child
                        .wait_with_output()
                        .await
                        .map_err(|e| format!("Failed to wait for node process: {}", e))?;

                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        return Ok(stdout.trim().to_string());
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("JavaScript execution failed: {}", stderr).into());
                    }
                }
                Err(e) => {
                    return Err(
                        format!("Failed to execute custom path '{}': {}", program, e).into(),
                    );
                }
            }
        }
    }

    // 获取用户主目录
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(target_os = "windows"))]
    let home = std::env::var("HOME").unwrap_or_default();

    // 常见的 JavaScript 运行环境路径
    #[cfg(target_os = "windows")]
    let paths: Vec<String> = {
        vec![
            "C:\\Program Files\\nodejs".to_string(),
            "C:\\Program Files (x86)\\nodejs".to_string(),
            format!("{}\\AppData\\Local\\Programs\\nodejs", home),
            format!("{}\\AppData\\Roaming\\npm", home),
            format!("{}\\.deno\\bin", home),
        ]
    };
    #[cfg(not(target_os = "windows"))]
    let paths: Vec<String> = {
        vec![
            "/usr/local/bin".to_string(),
            "/opt/homebrew/bin".to_string(),
            "/opt/local/bin".to_string(),
            "/usr/bin".to_string(),
            "/bin".to_string(),
            format!("{}/.local/bin", home),
            format!("{}/.deno/bin", home),
        ]
    };

    // 构建 PATH 环境变量
    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    let path = match std::env::var("PATH") {
        Ok(path) if !path.is_empty() => format!("{}{}{}", path, separator, paths.join(separator)),
        _ => paths.join(separator),
    };

    // 尝试使用 node，如果失败则尝试 deno
    let commands = [("node", vec!["-e"]), ("deno", vec!["eval"])];
    for (cmd, args) in &commands {
        let mut command = Command::new(cmd);
        for arg in args {
            command.arg(arg);
        }
        command
            .arg(&wrapped_code)
            .env("PATH", &path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        match command.spawn() {
            Ok(child) => {
                let output = child
                    .wait_with_output()
                    .await
                    .map_err(|e| format!("Failed to wait for {} process: {}", cmd, e))?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(stdout.trim().to_string());
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    // 如果是找不到命令的错误，继续尝试下一个命令
                    if stderr.contains("No such file or directory")
                        || stderr.contains("command not found")
                    {
                        continue;
                    }
                    return Err(format!("JavaScript execution failed: {}", stderr).into());
                }
            }
            Err(_) => continue, // 尝试下一个命令
        }
    }

    Err("JavaScript runtime not found. Please install Node.js or Deno.".into())
}

#[tauri::command]
async fn execute_python(
    code: String,
    data: String,
    python_path: Option<String>,
) -> Result<String, AppError> {
    // 创建 Python 代码包装
    let wrapped_code = format!(
        r#"
import json
data = {}
{}
result = process(data)
print(result if isinstance(result, str) else json.dumps(result, ensure_ascii=False))
        "#,
        data, code
    );

    // 如果提供了自定义路径，直接使用
    if let Some(program) = python_path {
        if !program.is_empty() {
            // 在 Windows 上，如果是 .bat 文件，需要特殊处理
            #[cfg(target_os = "windows")]
            let use_stdin = program.to_lowercase().ends_with(".bat");
            #[cfg(not(target_os = "windows"))]
            let use_stdin = false;

            let mut command = if use_stdin {
                // 对于 .bat 文件，通过 stdin 传递代码，避免参数转义问题
                let mut cmd = Command::new(&program);
                cmd.stdin(Stdio::piped());
                cmd
            } else {
                // 对于普通可执行文件，使用 -c 参数
                let mut cmd = Command::new(&program);
                cmd.arg("-c").arg(&wrapped_code);
                cmd
            };

            command
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            match command.spawn() {
                Ok(mut child) => {
                    // 如果使用 stdin，写入代码
                    if use_stdin {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(wrapped_code.as_bytes())
                                .await
                                .map_err(|e| format!("Failed to write to stdin: {}", e))?;
                            drop(stdin); // 关闭 stdin
                        }
                    }

                    let output = child
                        .wait_with_output()
                        .await
                        .map_err(|e| format!("Failed to wait for python process: {}", e))?;

                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        return Ok(stdout.trim().to_string());
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("Python execution failed: {}", stderr).into());
                    }
                }
                Err(e) => {
                    return Err(
                        format!("Failed to execute custom path '{}': {}", program, e).into(),
                    );
                }
            }
        }
    }

    // 获取用户主目录
    #[cfg(target_os = "windows")]
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(target_os = "windows"))]
    let home = std::env::var("HOME").unwrap_or_default();

    // 常见的 Python 运行环境路径
    #[cfg(target_os = "windows")]
    let paths: Vec<String> = {
        // 添加所有常见 Python 版本及其 Scripts 目录
        let mut paths = vec![format!("{}\\AppData\\Local\\Microsoft\\WindowsApps", home)];
        let versions = [
            "Python313",
            "Python312",
            "Python311",
            "Python310",
            "Python39",
        ];
        for version in versions {
            paths.push(format!(
                "{}\\AppData\\Local\\Programs\\Python\\{}",
                home, version
            ));
            paths.push(format!(
                "{}\\AppData\\Local\\Programs\\Python\\{}\\Scripts",
                home, version
            ));
            paths.push(format!("C:\\{}", version));
            paths.push(format!("C:\\{}\\Scripts", version));
        }
        paths
    };
    #[cfg(not(target_os = "windows"))]
    let paths: Vec<String> = {
        vec![
            format!("{}/.pyenv/shims", home),
            "/usr/local/bin".to_string(),
            "/opt/homebrew/bin".to_string(),
            "/opt/local/bin".to_string(),
            "/usr/bin".to_string(),
            "/bin".to_string(),
            format!("{}/.local/bin", home),
        ]
    };

    // 构建 PATH 环境变量
    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    let path = match std::env::var("PATH") {
        Ok(path) if !path.is_empty() => format!("{}{}{}", path, separator, paths.join(separator)),
        _ => paths.join(separator),
    };

    // 尝试使用 python3，如果失败则尝试 python
    let commands = ["python3", "python"];
    for cmd in &commands {
        let mut command = Command::new(cmd);
        command
            .arg("-c")
            .arg(&wrapped_code)
            .env("PATH", &path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        match command.spawn() {
            Ok(child) => {
                let output = child
                    .wait_with_output()
                    .await
                    .map_err(|e| format!("Failed to wait for python process: {}", e))?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    return Ok(stdout.trim().to_string());
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    // 如果是找不到命令的错误，继续尝试下一个命令
                    if stderr.contains("No such file or directory")
                        || stderr.contains("command not found")
                    {
                        continue;
                    }
                    return Err(format!("Python execution failed: {}", stderr).into());
                }
            }
            Err(_) => continue, // 尝试下一个命令
        }
    }

    Err("Python interpreter not found. Please install Python.".into())
}

#[tauri::command]
async fn register_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
    // 验证输入参数
    if key.len() != 1 || !key.chars().all(|c| c.is_alphanumeric()) {
        return Err("快捷键必须是单个字母或数字".into());
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
async fn unregister_shortcut(app: tauri::AppHandle, key: String) -> Result<(), AppError> {
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
async fn is_shortcut_registered(key: String) -> Result<bool, AppError> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        // 从 shortcut.key 格式化字符串中提取最后一个字符
                        let key_str = format!("{}", shortcut.key);
                        let key_char = key_str.chars().last().unwrap_or('?').to_string();

                        let app_clone = app.clone();
                        let key_char_clone = key_char.clone();

                        // 异步获取选中文本并发送事件到前端
                        tauri::async_runtime::spawn(async move {
                            match get_selection(app_clone.clone()).await {
                                Ok(selection) => {
                                    let event_data = serde_json::json!({
                                        "key": key_char_clone,
                                        "selection": selection
                                    });
                                    if let Err(e) = app_clone.emit("shortcut-triggered", event_data)
                                    {
                                        eprintln!("[ERROR] Failed to emit shortcut event: {}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[ERROR] Failed to get selection: {}", e);
                                    // 即使获取选中文本失败，也发送事件，但选中文本为空
                                    let event_data = serde_json::json!({
                                        "key": key_char_clone,
                                        "selection": ""
                                    });
                                    if let Err(e) = app_clone.emit("shortcut-triggered", event_data)
                                    {
                                        eprintln!("[ERROR] Failed to emit shortcut event: {}", e);
                                    }
                                }
                            }
                        });
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // 创建托盘菜单
            let menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(app, "toggle", "显示/隐藏窗口", true, None::<&str>)?,
                    &MenuItem::with_id(
                        app,
                        "shortcuts",
                        "编辑快捷键...",
                        true,
                        Some("CmdOrCtrl+,"),
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::about(app, Some("关于 TextGO"), None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrCtrl+Q"))?,
                ],
            )?;
            // 创建托盘图标
            let _tray = TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .show_menu_on_left_click(true) // 左键也显示菜单
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        // 调用 toggle_window 命令
                        toggle_window(app.clone());
                    }
                    "shortcuts" => {
                        // 调用 show_shortcuts 命令
                        show_shortcuts(app.clone());
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 获取主窗口
            if let Some(window) = app.get_webview_window("main") {
                let app_handle = window.app_handle().clone();

                // 由于窗口初始时是隐藏的，隐藏 Dock 图标
                #[cfg(target_os = "macos")]
                {
                    let _ = app.set_dock_visibility(false);
                }

                // 设置窗口关闭行为，关闭时隐藏而不是退出
                window.on_window_event(move |event| {
                    match event {
                        WindowEvent::CloseRequested { api, .. } => {
                            // 阻止默认关闭行为
                            api.prevent_close();
                            // 隐藏窗口到系统托盘
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.hide();
                                // 隐藏窗口时，隐藏 Dock 图标
                                #[cfg(target_os = "macos")]
                                {
                                    let _ = app_handle.set_dock_visibility(false);
                                }
                            }
                        }
                        _ => {}
                    }
                });
            }

            // 获取 popup 窗口并设置关闭行为
            if let Some(window) = app.get_webview_window("popup") {
                let app_handle = window.app_handle().clone();
                // 设置 popup 窗口关闭行为，关闭时隐藏而不是销毁
                window.on_window_event(move |event| {
                    match event {
                        WindowEvent::CloseRequested { api, .. } => {
                            // 阻止默认关闭行为
                            api.prevent_close();
                            // 隐藏窗口而不是销毁
                            if let Some(popup) = app_handle.get_webview_window("popup") {
                                let _ = popup.hide();
                            }
                        }
                        _ => {}
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_window,
            hide_window,
            toggle_window,
            show_shortcuts,
            send_copy_key,
            send_paste_key,
            get_selection,
            show_popup_window,
            execute_javascript,
            execute_python,
            register_shortcut,
            unregister_shortcut,
            is_shortcut_registered
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            // 处理 Reopen 事件
            #[cfg(target_os = "macos")]
            if let RunEvent::Reopen {
                has_visible_windows: false,
                ..
            } = event
            {
                // 没有可见窗口时，显示主窗口
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.unminimize();
                    // 显示窗口时，显示 Dock 图标
                    #[cfg(target_os = "macos")]
                    {
                        let _ = app_handle.set_dock_visibility(true);
                    }
                }
            }
        });
}
