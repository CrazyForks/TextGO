use crate::error::AppError;
use core_foundation::array::CFArray;
use core_foundation::base::{CFRange, CFType, CFTypeRef, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use std::os::raw::c_void;

// AXValueType constant
// https://developer.apple.com/documentation/applicationservices/axvaluetype/kaxvaluetypecfrange
const AX_VALUE_TYPE_CF_RANGE: i32 = 4;

// editable accessibility role list
const EDITABLE_AX_ROLES: &[&str] = &["AXTextField", "AXTextArea", "AXComboBox"];

// declare external functions from macOS ApplicationServices framework
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

/// Get UI element attribute value.
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

/// Set UI element attribute value.
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

/// Get currently focused UI element.
fn get_focused_element() -> Result<CFType, AppError> {
    unsafe {
        // check accessibility permission
        if !AXIsProcessTrusted() {
            return Err("Accessibility permission not granted".into());
        }

        // create system-wide AXUIElement
        let sys_element_ptr = AXUIElementCreateSystemWide();
        if sys_element_ptr.is_null() {
            return Err("Failed to create system-wide AXUIElement".into());
        }
        let sys_element = CFType::wrap_under_create_rule(sys_element_ptr);

        // get focused element
        get_element_attribute(&sys_element, "AXFocusedUIElement")
    }
}

/// Get selected text attribute from given element.
fn get_selected_text(element: &CFType) -> Option<String> {
    // try to get selected text
    if let Ok(selected_text) = get_element_attribute(element, "AXSelectedText") {
        if let Some(text) = selected_text.downcast::<CFString>() {
            return Some(text.to_string());
        }
    }
    None
}

/// Get selected text in currently focused element.
pub fn get_selection() -> Result<String, AppError> {
    // get focused element
    let focused_element = get_focused_element()?;

    // 1. try to get selected text directly from focused element
    if let Some(text) = get_selected_text(&focused_element) {
        return Ok(text);
    }

    // 2. if focused element has no selected text, try traversing child elements
    if let Ok(ax_children) = get_element_attribute(&focused_element, "AXChildren") {
        if let Some(children) = ax_children.downcast::<CFArray>() {
            // traverse child element array
            for i in 0..children.len() {
                unsafe {
                    // get raw pointer from array directly
                    if let Some(child_ptr) = children.get(i).map(|item| *item as CFTypeRef) {
                        if !child_ptr.is_null() {
                            // wrap pointer as CFType
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

    // if not found, return empty string
    Ok(String::new())
}

/// Check if currently focused element is editable.
pub fn is_cursor_editable() -> Result<bool, AppError> {
    // get focused element
    let focused_element = get_focused_element()?;

    // get role of focused element
    let ax_role = get_element_attribute(&focused_element, "AXRole")?;

    // check if role is editable type
    Ok(ax_role.downcast::<CFString>().is_some_and(|role| {
        EDITABLE_AX_ROLES
            .iter()
            .any(|r| role.to_string().contains(r))
    }))
}

/// Select specified number of characters from current cursor position backward.
pub fn select_backward_chars(chars: usize) -> Result<(), AppError> {
    unsafe {
        // get focused element
        let focused_element = get_focused_element()?;

        // get currently selected text range
        let text_range = get_element_attribute(&focused_element, "AXSelectedTextRange")?;

        // extract raw CFRange structure from AXValue
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

        // calculate new selection range
        let new_range = CFRange {
            location: (orig_range.location + orig_range.length - chars as isize).max(0),
            length: chars as isize,
        };

        // create AXValue object
        let new_range_ptr = AXValueCreate(AX_VALUE_TYPE_CF_RANGE, &new_range as *const _ as _);
        if new_range_ptr.is_null() {
            return Err("Failed to create new range AXValue".into());
        }
        let new_range_value = CFType::wrap_under_create_rule(new_range_ptr);

        // set new selection range
        set_element_attribute(
            &focused_element,
            "AXSelectedTextRange",
            new_range_value.as_CFTypeRef(),
        )?;

        Ok(())
    }
}
