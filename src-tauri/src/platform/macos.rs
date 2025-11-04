use crate::error::AppError;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use std::ffi::c_void;

/// 获取当前焦点元素中用户选中的文本 (macOS 实现)
///
/// 通过 Accessibility API 获取焦点元素的选中文本
/// 获取失败时返回空字符串
pub fn get_selected_text() -> Result<String, AppError> {
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
            return Err("Accessibility permission not granted".into());
        }

        // 创建系统级别的 AXUIElement
        let sys_el = AXUIElementCreateSystemWide();
        if sys_el.is_null() {
            return Err("Failed to create system-wide AXUIElement".into());
        }

        // 获取当前焦点元素
        let mut focused_el: *const c_void = std::ptr::null();
        let focused_el_result = AXUIElementCopyAttributeValue(
            sys_el,
            CFString::new("AXFocusedUIElement").as_concrete_TypeRef() as *const c_void,
            &mut focused_el,
        );
        CFRelease(sys_el);
        if focused_el_result != 0 {
            return Err(format!("Failed to get focused element, error code: {}", focused_el_result).into());
        }
        if focused_el.is_null() {
            return Err("No focused element found".into());
        }

        // 获取选中的文本
        let mut selected_text: *const c_void = std::ptr::null();
        let selected_text_result = AXUIElementCopyAttributeValue(
            focused_el,
            CFString::new("AXSelectedText").as_concrete_TypeRef() as *const c_void,
            &mut selected_text,
        );

        let result = if selected_text_result == 0 && !selected_text.is_null() {
            // 将 CFString 转换为 Rust String
            let text = selected_text as *const core_foundation::string::__CFString;
            let text_string = CFString::wrap_under_get_rule(text).to_string();
            CFRelease(selected_text);
            text_string
        } else {
            CFRelease(focused_el);
            if selected_text_result != 0 {
                return Err(format!("Failed to get selected text, error code: {}", selected_text_result).into());
            } else {
                return Err("No text selected".into());
            }
        };

        CFRelease(focused_el);
        Ok(result)
    }
}
