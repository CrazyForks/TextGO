use crate::error::AppError;
use windows::core::Interface;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationLegacyIAccessiblePattern,
    IUIAutomationTextPattern, IUIAutomationTextRangeArray, IUIAutomationValuePattern,
    TextPatternRangeEndpoint_Start, TextUnit_Character, UIA_DocumentControlTypeId,
    UIA_EditControlTypeId, UIA_LegacyIAccessiblePatternId, UIA_TextPatternId, UIA_ValuePatternId,
};

/// COM resource guard
struct ComGuard {
    initialized: bool,
}

impl ComGuard {
    /// initialize COM environment
    fn new() -> Result<Self, AppError> {
        unsafe {
            let result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if result.is_ok() {
                // successfully initialized, need to release on drop
                Ok(ComGuard { initialized: true })
            } else if result == windows::Win32::Foundation::RPC_E_CHANGED_MODE {
                // COM already initialized by other code, no need to release
                Ok(ComGuard { initialized: false })
            } else {
                // other errors
                Err("Failed to initialize COM".into())
            }
        }
    }
}

impl Drop for ComGuard {
    /// release COM resources
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// get focused element
fn get_focused_element() -> Result<IUIAutomationElement, AppError> {
    unsafe {
        // create UI Automation instance
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create UI Automation instance: {}", e))?;

        // get focused element
        automation
            .GetFocusedElement()
            .map_err(|e| format!("Failed to get focused element: {}", e).into())
    }
}

/// get text selection range from focused element
fn get_text_selection(
    focused_element: &IUIAutomationElement,
) -> Result<IUIAutomationTextRangeArray, AppError> {
    unsafe {
        // get text pattern from focused element
        let text_pattern: IUIAutomationTextPattern = focused_element
            .GetCurrentPattern(UIA_TextPatternId)
            .and_then(|p| p.cast())
            .map_err(|_| "Failed to get text pattern")?;

        // get current selected text range
        let selection = text_pattern
            .GetSelection()
            .map_err(|_| "Failed to get text selection")?;

        if selection.Length().unwrap_or(0) == 0 {
            return Err("No text selection found".into());
        }

        Ok(selection)
    }
}

/// get selected text by user in current focused element
pub fn get_selection() -> Result<String, AppError> {
    unsafe {
        // initialize COM
        let _com = ComGuard::new()?;

        // get focused element
        let focused_element = get_focused_element()?;

        // get current selected text range
        let selection = get_text_selection(&focused_element)?;

        // get first selection range and extract text
        let text = selection
            .GetElement(0)
            .and_then(|e| e.GetText(-1))
            .map_err(|_| "Failed to get text from selection")?;

        Ok(text.to_string())
    }
}

/// check if current focused element is editable
pub fn is_cursor_editable() -> Result<bool, AppError> {
    unsafe {
        // initialize COM
        let _com = ComGuard::new()?;

        // get focused element
        let focused_element = get_focused_element()?;

        // 1. check if control type is editable
        if let Ok(control_type) = focused_element.CurrentControlType() {
            let is_edit_control = control_type.0 == UIA_EditControlTypeId.0;
            let is_document_control = control_type.0 == UIA_DocumentControlTypeId.0;
            if is_edit_control || is_document_control {
                return Ok(true);
            }
        }

        // 2. check if pattern is not readonly
        if let Ok(is_readonly) = focused_element
            .GetCurrentPattern(UIA_ValuePatternId)
            .and_then(|p| p.cast::<IUIAutomationValuePattern>())
            .and_then(|vp| vp.CurrentIsReadOnly())
        {
            if !is_readonly.as_bool() {
                return Ok(true);
            }
        }

        // 3. check legacy control role
        if let Ok(role) = focused_element
            .GetCurrentPattern(UIA_LegacyIAccessiblePatternId)
            .and_then(|p| p.cast::<IUIAutomationLegacyIAccessiblePattern>())
            .and_then(|lp| lp.CurrentRole())
        {
            // ROLE_SYSTEM_TEXT (42) means editable text
            // ROLE_SYSTEM_COMBOBOX (46) means combo box
            if role == 42 || role == 46 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

/// select specified number of characters from current cursor position backward
pub fn select_backward_chars(chars: usize) -> Result<(), AppError> {
    unsafe {
        // initialize COM
        let _com = ComGuard::new()?;

        // get focused element
        let focused_element = get_focused_element()?;

        // get current selected text range
        let selection = get_text_selection(&focused_element)?;

        // get first selection range
        let text_range = selection
            .GetElement(0)
            .map_err(|_| "Failed to get selection range")?;

        // move start point backward
        text_range
            .MoveEndpointByUnit(
                TextPatternRangeEndpoint_Start,
                TextUnit_Character,
                -(chars as i32),
            )
            .map_err(|_| "Failed to move endpoint backward")?;

        // select new range
        text_range
            .Select()
            .map_err(|_| "Failed to select new range")?;

        Ok(())
    }
}
