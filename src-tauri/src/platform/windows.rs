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

/// COM 资源守护
struct ComGuard {
    initialized: bool,
}

impl ComGuard {
    /// 初始化 COM 环境
    fn new() -> Result<Self, AppError> {
        unsafe {
            let result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if result.is_ok() {
                // 成功初始化，需要在 drop 时释放
                Ok(ComGuard { initialized: true })
            } else if result == windows::Win32::Foundation::RPC_E_CHANGED_MODE {
                // COM 已被其他代码初始化，不需要释放
                Ok(ComGuard { initialized: false })
            } else {
                // 其他错误
                Err("Failed to initialize COM".into())
            }
        }
    }
}

impl Drop for ComGuard {
    /// 释放 COM 资源
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// 获取当前焦点元素
fn get_focused_element() -> Result<IUIAutomationElement, AppError> {
    unsafe {
        // 创建 UI Automation 实例
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create UI Automation instance: {}", e))?;

        // 获取当前焦点元素
        automation
            .GetFocusedElement()
            .map_err(|e| format!("Failed to get focused element: {}", e).into())
    }
}

/// 从焦点元素获取选中的文本范围
fn get_text_selection(
    focused_element: &IUIAutomationElement,
) -> Result<IUIAutomationTextRangeArray, AppError> {
    unsafe {
        // 获取焦点元素的 TextPattern
        let text_pattern: IUIAutomationTextPattern = focused_element
            .GetCurrentPattern(UIA_TextPatternId)
            .and_then(|p| p.cast())
            .map_err(|_| "Failed to get text pattern")?;

        // 获取当前选中的文本范围
        let selection = text_pattern
            .GetSelection()
            .map_err(|_| "Failed to get text selection")?;

        if selection.Length().unwrap_or(0) == 0 {
            return Err("No text selection found".into());
        }

        Ok(selection)
    }
}

/// 获取当前焦点元素中用户选中的文本
pub fn get_selection() -> Result<String, AppError> {
    unsafe {
        // 初始化 COM
        let _com = ComGuard::new()?;

        // 获取当前焦点元素
        let focused_element = get_focused_element()?;

        // 获取当前选中的文本范围
        let selection = get_text_selection(&focused_element)?;

        // 获取第一个选中范围并提取文本
        let text = selection
            .GetElement(0)
            .and_then(|e| e.GetText(-1))
            .map_err(|_| "Failed to get text from selection")?;

        Ok(text.to_string())
    }
}

/// 检查当前焦点元素是否可编辑
pub fn is_cursor_editable() -> Result<bool, AppError> {
    unsafe {
        // 初始化 COM
        let _com = ComGuard::new()?;

        // 获取当前焦点元素
        let focused_element = get_focused_element()?;

        // 1. 检查控件类型是否为可编辑
        if let Ok(control_type) = focused_element.CurrentControlType() {
            let is_edit_control = control_type.0 == UIA_EditControlTypeId.0;
            let is_document_control = control_type.0 == UIA_DocumentControlTypeId.0;
            if is_edit_control || is_document_control {
                return Ok(true);
            }
        }

        // 2. 检查模式是否不为只读
        if let Ok(is_readonly) = focused_element
            .GetCurrentPattern(UIA_ValuePatternId)
            .and_then(|p| p.cast::<IUIAutomationValuePattern>())
            .and_then(|vp| vp.CurrentIsReadOnly())
        {
            if !is_readonly.as_bool() {
                return Ok(true);
            }
        }

        // 3. 检查老式控件的角色
        if let Ok(role) = focused_element
            .GetCurrentPattern(UIA_LegacyIAccessiblePatternId)
            .and_then(|p| p.cast::<IUIAutomationLegacyIAccessiblePattern>())
            .and_then(|lp| lp.CurrentRole())
        {
            // ROLE_SYSTEM_TEXT (42) 表示可编辑文本
            // ROLE_SYSTEM_COMBOBOX (46) 表示组合框
            if role == 42 || role == 46 {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

/// 从当前光标位置开始向左选中指定数量的字符
pub fn select_backward_chars(chars: usize) -> Result<(), AppError> {
    unsafe {
        // 初始化 COM
        let _com = ComGuard::new()?;

        // 获取当前焦点元素
        let focused_element = get_focused_element()?;

        // 获取当前选中的文本范围
        let selection = get_text_selection(&focused_element)?;

        // 获取第一个选中范围
        let text_range = selection
            .GetElement(0)
            .map_err(|_| "Failed to get selection range")?;

        // 向左移动起始点
        text_range
            .MoveEndpointByUnit(
                TextPatternRangeEndpoint_Start,
                TextUnit_Character,
                -(chars as i32),
            )
            .map_err(|_| "Failed to move endpoint backward")?;

        // 选中新范围
        text_range
            .Select()
            .map_err(|_| "Failed to select new range")?;

        Ok(())
    }
}
