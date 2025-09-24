use crate::error::AppError;
use core_foundation::array::CFArray;
use core_foundation::base::{CFRange, CFType, CFTypeRef, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use std::os::raw::c_void;

// AXValueType 常量
// https://developer.apple.com/documentation/applicationservices/axvaluetype/kaxvaluetypecfrange
const AX_VALUE_TYPE_CF_RANGE: i32 = 4;

// 可编辑的 Accessibility 角色列表
const EDITABLE_AX_ROLES: &[&str] = &["AXTextField", "AXTextArea", "AXComboBox"];

// 声明 macOS ApplicationServices 框架中的外部函数
#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    unsafe fn AXIsProcessTrusted() -> bool;

    unsafe fn AXUIElementCreateSystemWide() -> CFTypeRef;

    unsafe fn AXUIElementCopyAttributeValue(
        element: CFTypeRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> i32;

    unsafe fn AXUIElementSetAttributeValue(
        element: CFTypeRef,
        attribute: CFStringRef,
        value: CFTypeRef,
    ) -> i32;

    unsafe fn AXValueCreate(value_type: i32, value_ptr: *const c_void) -> CFTypeRef;

    unsafe fn AXValueGetValue(value: CFTypeRef, value_type: i32, value_ptr: *mut c_void) -> bool;
}

/// 获取 UI 元素的属性值
fn get_element_attribute(element: &CFType, attribute: &str) -> Result<CFType, AppError> {
    unsafe {
        let mut value_ptr: CFTypeRef = std::ptr::null();
        let error_code = AXUIElementCopyAttributeValue(
            element.as_CFTypeRef(),
            CFString::new(attribute).as_concrete_TypeRef(),
            &mut value_ptr,
        );

        if error_code != 0 || value_ptr.is_null() {
            return Err(format!("Failed to get {}, error code: {}", attribute, error_code).into());
        }

        Ok(CFType::wrap_under_create_rule(value_ptr))
    }
}

/// 设置 UI 元素的属性值
fn set_element_attribute(
    element: &CFType,
    attribute: &str,
    value: CFTypeRef,
) -> Result<(), AppError> {
    unsafe {
        let error_code = AXUIElementSetAttributeValue(
            element.as_CFTypeRef(),
            CFString::new(attribute).as_concrete_TypeRef(),
            value,
        );

        if error_code != 0 {
            return Err(format!("Failed to set {}, error code: {}", attribute, error_code).into());
        }

        Ok(())
    }
}

/// 获取当前焦点元素
fn get_focused_element() -> Result<CFType, AppError> {
    unsafe {
        // 检查是否有 Accessibility 权限
        if !AXIsProcessTrusted() {
            return Err("Accessibility permission not granted".into());
        }

        // 创建系统级别的 AXUIElement
        let sys_element_ptr = AXUIElementCreateSystemWide();
        if sys_element_ptr.is_null() {
            return Err("Failed to create system-wide AXUIElement".into());
        }
        let sys_element = CFType::wrap_under_create_rule(sys_element_ptr);

        // 获取当前焦点元素
        get_element_attribute(&sys_element, "AXFocusedUIElement")
    }
}

/// 从元素中获取选中的文本
fn get_selected_text(element: &CFType) -> Option<String> {
    // 尝试获取选中的文本
    if let Ok(selected_text) = get_element_attribute(element, "AXSelectedText") {
        if let Some(text) = selected_text.downcast::<CFString>() {
            return Some(text.to_string());
        }
    }
    None
}

/// 获取当前焦点元素中用户选中的文本
pub fn get_selection() -> Result<String, AppError> {
    // 获取当前焦点元素
    let focused_element = get_focused_element()?;

    // 1. 尝试直接从焦点元素获取选中的文本
    if let Some(text) = get_selected_text(&focused_element) {
        return Ok(text);
    }

    // 2. 如果焦点元素没有选中文本，尝试遍历子元素
    if let Ok(ax_children) = get_element_attribute(&focused_element, "AXChildren") {
        if let Some(children) = ax_children.downcast::<CFArray>() {
            // 遍历子元素数组
            for i in 0..children.len() {
                unsafe {
                    // 直接获取数组中的原始指针
                    if let Some(child_ptr) = children.get(i).map(|item| *item as CFTypeRef) {
                        if !child_ptr.is_null() {
                            // 包装指针为 CFType
                            let child_element = CFType::wrap_under_get_rule(child_ptr);
                            if let Some(text) = get_selected_text(&child_element) {
                                return Ok(text);
                            }
                        }
                    }
                }
            }
        }
    }

    // 如果都没有找到，返回空字符串
    Ok(String::new())
}

/// 检查当前焦点元素是否可编辑
pub fn is_cursor_editable() -> Result<bool, AppError> {
    // 获取当前焦点元素
    let focused_element = get_focused_element()?;

    // 获取焦点元素的角色
    let ax_role = get_element_attribute(&focused_element, "AXRole")?;

    // 检查角色是否为可编辑类型
    Ok(ax_role.downcast::<CFString>().is_some_and(|role| {
        EDITABLE_AX_ROLES
            .iter()
            .any(|r| role.to_string().contains(r))
    }))
}

/// 从当前光标位置开始向左选中指定数量的字符
pub fn select_backward_chars(chars: usize) -> Result<(), AppError> {
    unsafe {
        // 获取当前焦点元素
        let focused_element = get_focused_element()?;

        // 获取当前选中文本的范围
        let text_range = get_element_attribute(&focused_element, "AXSelectedTextRange")?;

        // 从 AXValue 中提取原始的 CFRange 结构体
        let mut orig_range = CFRange {
            location: 0,
            length: 0,
        };
        let get_orig_success = AXValueGetValue(
            text_range.as_CFTypeRef(),
            AX_VALUE_TYPE_CF_RANGE,
            &mut orig_range as *mut _ as _,
        );
        if !get_orig_success {
            return Err("Failed to get selection range".into());
        }

        // 计算新的选中范围
        let new_range = CFRange {
            location: (orig_range.location + orig_range.length - chars as isize).max(0),
            length: chars as isize,
        };

        // 创建 AXValue 对象
        let new_range_ptr = AXValueCreate(AX_VALUE_TYPE_CF_RANGE, &new_range as *const _ as _);
        if new_range_ptr.is_null() {
            return Err("Failed to create new range AXValue".into());
        }
        let new_range_value = CFType::wrap_under_create_rule(new_range_ptr);

        // 设置新的选中范围
        set_element_attribute(
            &focused_element,
            "AXSelectedTextRange",
            new_range_value.as_CFTypeRef(),
        )?;

        Ok(())
    }
}
