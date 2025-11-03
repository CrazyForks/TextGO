use crate::error::AppError;

/// 检查当前焦点元素是否可编辑
///
/// 在 macOS 上，通过 Accessibility API 检查当前焦点元素的角色
/// 在 Windows 上，通过 UI Automation 检查当前焦点元素的控件类型或模式
///
/// 获取焦点元素失败或无法准确判断时，默认返回 true
#[tauri::command]
pub fn is_editable() -> Result<bool, AppError> {
    #[cfg(target_os = "macos")]
    match std::panic::catch_unwind(is_editable_macos) {
        Ok(result) => result,
        Err(_) => Ok(false),
    }

    #[cfg(target_os = "windows")]
    match std::panic::catch_unwind(is_editable_windows) {
        Ok(result) => result,
        Err(_) => Ok(false),
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    Err("Unsupported platform".into())
}

#[cfg(target_os = "macos")]
fn is_editable_macos() -> Result<bool, AppError> {
    use core_foundation::base::TCFType;
    use core_foundation::string::CFString;
    use std::ffi::c_void;

    // Accessibility API 的外部函数声明
    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        unsafe fn AXIsProcessTrusted() -> bool;
        unsafe fn AXUIElementCreateSystemWide() -> *const c_void;
        unsafe fn AXUIElementCopyAttributeValue(
            element: *const c_void,
            attribute: *const c_void,
            value: *mut *const c_void,
        ) -> i32;
        unsafe fn CFRelease(cf: *const c_void);
    }

    unsafe {
        // 检查是否有 Accessibility 权限
        if !AXIsProcessTrusted() {
            return Ok(true);
        }

        // 创建系统级别的 AXUIElement
        let sys_el = AXUIElementCreateSystemWide();
        if sys_el.is_null() {
            return Ok(true);
        }

        // 获取当前焦点元素
        let mut focused_el: *const c_void = std::ptr::null();
        let focused_el_result = AXUIElementCopyAttributeValue(
            sys_el,
            CFString::new("AXFocusedUIElement").as_concrete_TypeRef() as *const c_void,
            &mut focused_el,
        );
        CFRelease(sys_el);
        if focused_el_result != 0 || focused_el.is_null() {
            return Ok(true);
        }

        // 获取焦点元素的角色
        let mut ax_role: *const c_void = std::ptr::null();
        let ax_role_result = AXUIElementCopyAttributeValue(
            focused_el,
            CFString::new("AXRole").as_concrete_TypeRef() as *const c_void,
            &mut ax_role,
        );

        // 检查角色是否为可编辑类型
        let mut editable = false;
        if ax_role_result == 0 && !ax_role.is_null() {
            // 将 CFString 转换为 Rust String
            let role = ax_role as *const core_foundation::string::__CFString;
            let role_string = match std::panic::catch_unwind(|| {
                CFString::wrap_under_get_rule(role).to_string()
            }) {
                Ok(s) => s,
                Err(_) => {
                    CFRelease(ax_role);
                    CFRelease(focused_el);
                    return Ok(editable);
                }
            };

            // 检查是否为可编辑的角色
            let editable_roles = vec![
                "AXTextField", // 文本输入框
                "AXTextArea",  // 文本输入区域
                "AXComboBox",  // 组合框
                "AXWindow",    // 窗口（某些应用可能使用自绘控件）
            ];
            for role in &editable_roles {
                if role_string.contains(role) {
                    editable = true;
                    break;
                }
            }
            CFRelease(ax_role);
        }
        CFRelease(focused_el);
        Ok(editable)
    }
}

#[cfg(target_os = "windows")]
fn is_editable_windows() -> Result<bool, AppError> {
    use windows::core::Interface;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
    };
    use windows::Win32::UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationElement, UIA_DocumentControlTypeId,
        UIA_EditControlTypeId, UIA_ValuePatternId,
    };

    unsafe {
        // 初始化 COM
        let result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if result.is_err() && result != windows::Win32::Foundation::RPC_E_CHANGED_MODE {
            return Ok(true);
        }

        // 创建 UI Automation 实例
        let automation: IUIAutomation = match CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) {
            Ok(auto) => auto,
            Err(_) => {
                CoUninitialize();
                return Ok(true);
            }
        };

        // 获取当前焦点元素
        let focused_el: IUIAutomationElement = match automation.GetFocusedElement() {
            Ok(el) => el,
            Err(_) => {
                CoUninitialize();
                return Ok(true);
            }
        };

        // 1. 检查控件类型是否为可编辑
        if let Ok(control_type) = focused_el.CurrentControlType() {
            let is_edit_control = control_type.0 == UIA_EditControlTypeId.0;
            let is_document_control = control_type.0 == UIA_DocumentControlTypeId.0;
            if is_edit_control || is_document_control {
                CoUninitialize();
                return Ok(true);
            }
        }

        // 2. 检查模式是否不为只读
        if let Ok(pattern) = focused_el.GetCurrentPattern(UIA_ValuePatternId) {
            use windows::Win32::UI::Accessibility::IUIAutomationValuePattern;

            if let Ok(value_pattern) = pattern.cast::<IUIAutomationValuePattern>() {
                if let Ok(is_readonly) = value_pattern.CurrentIsReadOnly() {
                    if !is_readonly.as_bool() {
                        CoUninitialize();
                        return Ok(true);
                    }
                }
            }
        }

        // 3. 检查老式控件的角色
        use windows::Win32::UI::Accessibility::UIA_LegacyIAccessiblePatternId;
        if let Ok(pattern) = focused_el.GetCurrentPattern(UIA_LegacyIAccessiblePatternId) {
            use windows::Win32::UI::Accessibility::IUIAutomationLegacyIAccessiblePattern;

            if let Ok(legacy_pattern) = pattern.cast::<IUIAutomationLegacyIAccessiblePattern>() {
                if let Ok(role) = legacy_pattern.CurrentRole() {
                    // ROLE_SYSTEM_TEXT (42) 表示可编辑文本
                    // ROLE_SYSTEM_COMBOBOX (46) 表示组合框
                    if role == 42 || role == 46 {
                        CoUninitialize();
                        return Ok(true);
                    }
                }
            }
        }

        CoUninitialize();
        Ok(false)
    }
}
