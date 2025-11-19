use crate::error::AppError;

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn AXIsProcessTrusted() -> bool;
}

/// Check if the application has accessibility permissions on macOS.
#[tauri::command]
pub fn check_accessibility() -> Result<bool, AppError> {
    #[cfg(target_os = "macos")]
    {
        unsafe { Ok(AXIsProcessTrusted()) }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // always return true on non-macOS platforms
        Ok(true)
    }
}

/// Open macOS accessibility settings page.
#[tauri::command]
pub fn open_accessibility() -> Result<(), AppError> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()
            .map_err(|e| format!("Failed to open accessibility settings: {}", e))?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("Accessibility settings are only available on macOS".into())
    }
}
