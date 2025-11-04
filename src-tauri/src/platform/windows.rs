use crate::error::AppError;
use windows::core::Interface;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTextPattern, UIA_TextPatternId,
};

/// COM 资源守护，确保在离开作用域时自动调用 CoUninitialize
struct ComGuard;

impl Drop for ComGuard {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

/// 获取当前焦点元素中用户选中的文本 (Windows 实现)
///
/// 通过 UI Automation 获取焦点元素的选中文本
/// 获取失败时返回错误
pub fn get_selected_text() -> Result<String, AppError> {
    unsafe {
        // 初始化 COM
        let result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if result.is_err() && result != windows::Win32::Foundation::RPC_E_CHANGED_MODE {
            return Err("Failed to initialize COM".into());
        }

        let _guard = ComGuard;

        // 创建 UI Automation 实例
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)
            .map_err(|e| format!("Failed to create UI Automation instance: {}", e))?;

        // 获取当前焦点元素
        let focused_el: IUIAutomationElement = automation
            .GetFocusedElement()
            .map_err(|e| format!("Failed to get focused element: {}", e))?;

        // 获取 Text Pattern
        let pattern = focused_el
            .GetCurrentPattern(UIA_TextPatternId)
            .map_err(|_| "Element does not support text pattern")?;

        let text_pattern = pattern
            .cast::<IUIAutomationTextPattern>()
            .map_err(|_| "Failed to cast to text pattern")?;

        // 获取选中的文本范围
        let selection_array = text_pattern
            .GetSelection()
            .map_err(|_| "Failed to get text selection")?;

        let length = selection_array
            .Length()
            .map_err(|_| "Failed to get selection array length")?;

        if length == 0 {
            return Err("No text selected".into());
        }

        // 获取第一个选中范围
        let range = selection_array
            .GetElement(0)
            .map_err(|_| "Failed to get selection range element")?;

        let text = range
            .GetText(-1)
            .map_err(|_| "Failed to get text from selection range")?;

        Ok(text.to_string())
    }
}
