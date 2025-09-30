import { invoke } from '@tauri-apps/api/core';
import { clear, readText, writeText } from '@tauri-apps/plugin-clipboard-manager';

/**
 * 获取剪贴板中的文本内容
 *
 * @returns 剪贴板中的文本，如果读取失败则返回空字符串
 */
export async function getClipboardText(): Promise<string> {
  try {
    return await readText();
  } catch (error) {
    console.error('读取剪贴板内容失败:', error);
    return '';
  }
}

/**
 * 设置剪贴板中的文本内容
 *
 * @param text - 要设置到剪贴板的文本内容
 */
export async function setClipboardText(text: string): Promise<void> {
  try {
    await writeText(text);
  } catch (error) {
    console.error('写入剪贴板内容失败:', error);
  }
}

/**
 * 获取当前选中的文本内容 (Rust版本)
 *
 * @returns 选中的文本内容，如果没有选中任何文本或操作失败则返回原剪贴板内容
 */
export async function getSelectedTextRust(): Promise<string> {
  try {
    return await invoke('get_selected_text');
  } catch (error) {
    console.error('获取选中文本失败 (Rust版本):', error);
    return '';
  }
}

/**
 * 获取当前选中的文本内容 (TypeScript版本)
 *
 * @returns 选中的文本内容，如果没有选中任何文本或操作失败则返回原剪贴板内容
 */
export async function getSelectedText(): Promise<string> {
  try {
    // 保存当前剪贴板内容
    const originalClipboard = await getClipboardText();
    // 清空剪贴板内容
    await clear();
    // 发送 Cmd+C 快捷键来复制选中的文字
    await invoke('send_copy_key');

    // 循环等待剪贴板内容变化
    const maxWaitTime = 500; // 最大等待时间 500ms
    const checkInterval = 50; // 每次检查间隔 50ms
    const maxAttempts = Math.floor(maxWaitTime / checkInterval);

    for (let attempt = 0; attempt < maxAttempts; attempt++) {
      await new Promise((resolve) => setTimeout(resolve, checkInterval));

      // 读取当前剪贴板内容
      const currentClipboard = await getClipboardText();

      // 如果剪贴板内容发生了变化，说明复制操作完成
      if (currentClipboard) {
        if (originalClipboard) {
          // 恢复原来的剪贴板内容
          await setClipboardText(originalClipboard);
        }
        return currentClipboard;
      }
    }

    // 超时后仍然没有变化，可能没有选中任何文字
    console.warn(`剪贴板内容在 ${maxWaitTime}ms 内未发生变化，可能没有选中任何文本`);
    return originalClipboard;
  } catch (error) {
    console.error('获取选中文本失败:', error);
    return '';
  }
}
