use crate::error::AppError;
use std::process::Stdio;
use tokio::{io::AsyncWriteExt, process::Command};

#[tauri::command]
pub async fn execute_javascript(
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

            command.stdout(Stdio::piped()).stderr(Stdio::piped());

            match command.spawn() {
                Ok(mut child) => {
                    // 如果使用 stdin，写入代码
                    if use_stdin {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(wrapped_code.as_bytes()).await?;
                            drop(stdin); // 关闭 stdin
                        }
                    }

                    let output = child.wait_with_output().await?;

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
                        format!("Failed to execute the program at '{}': {}", program, e).into(),
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
                let output = child.wait_with_output().await?;

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
pub async fn execute_python(
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

            command.stdout(Stdio::piped()).stderr(Stdio::piped());

            match command.spawn() {
                Ok(mut child) => {
                    // 如果使用 stdin，写入代码
                    if use_stdin {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(wrapped_code.as_bytes()).await?;
                            drop(stdin); // 关闭 stdin
                        }
                    }

                    let output = child.wait_with_output().await?;

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
                        format!("Failed to execute the program at '{}': {}", program, e).into(),
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
                let output = child.wait_with_output().await?;

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
