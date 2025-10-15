import { execute } from '$lib/executor';
import { match } from '$lib/matcher';
import { shortcuts } from '$lib/states.svelte';
import type { Hotkey } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * 快捷键管理类
 */
export class Manager {
  /**
   * 事件监听器是否已初始化
   */
  private listenerInitialized: boolean = false;

  /**
   * 初始化事件监听器
   */
  private async initializeListener(): Promise<void> {
    if (this.listenerInitialized) {
      return;
    }

    try {
      // 监听来自 Rust 后端的快捷键触发事件
      await listen('shortcut-triggered', async (event) => {
        const payload = event.payload as { key: string; selectedText: string };
        console.debug('------------------------------');
        console.debug(`接收到快捷键触发事件: ${payload.key}`);
        console.debug(
          `获取到选中文本: ${payload.selectedText.slice(0, 20)}${payload.selectedText.length > 20 ? '...' : ''}`
        );
        await this.handleShortcutEvent(payload.key, payload.selectedText);
      });

      this.listenerInitialized = true;
      console.debug('快捷键事件监听器初始化成功');
    } catch (error) {
      console.error('初始化快捷键事件监听器失败:', error);
    }
  }

  /**
   * 处理快捷键事件
   *
   * @param key - 触发的快捷键
   * @param selectedText - 已获取的选中文本
   */
  private async handleShortcutEvent(key: string, selectedText: string): Promise<void> {
    try {
      // 获取所有绑定到该 shortcut 的 hotkey
      const hotkeys = shortcuts.current[key];
      if (!hotkeys || hotkeys.length === 0) {
        return;
      }
      // 使用从后端传来的选中文本
      const text = selectedText;
      const hotkey = await match(text, hotkeys);
      if (hotkey === null) {
        console.warn('没有匹配的快捷键动作');
        return;
      }
      // 执行默认动作
      if (hotkey.action === '') {
        await this.showWindow();
        return;
      }
      // 执行匹配到的动作
      await execute(hotkey, text);
    } catch (error) {
      console.error(`执行快捷键动作失败: ${key}`, error);
    }
  }

  /**
   * 注册全局快捷键
   *
   * @param hotkey - 快捷键配置
   */
  async register(hotkey: Hotkey): Promise<void> {
    // 确保事件监听器已初始化
    await this.initializeListener();

    // 一个 shortcut 对应多个 hotkey
    try {
      // 检查 shortcut 是否已经在后端注册
      const isRegistered = await invoke('is_shortcut_registered', { key: hotkey.key });
      if (!isRegistered) {
        // 调用后端注册快捷键
        await invoke('register_shortcut', { key: hotkey.key });
      }
      // 保存 hotkey 到前端注册表中
      const hotkeys = shortcuts.current[hotkey.key];
      if (hotkeys && !hotkeys.find((h) => h.id === hotkey.id)) {
        hotkeys.push(hotkey);
      }
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  /**
   * 注销指定快捷键
   *
   * @param hotkey - 快捷键配置
   */
  async unregister(hotkey: Hotkey): Promise<void> {
    try {
      // 从前端注册表中移除 hotkey
      const hotkeys = shortcuts.current[hotkey.key];
      if (hotkeys) {
        const index = hotkeys.findIndex((h) => h.id === hotkey.id);
        if (index !== -1) {
          hotkeys.splice(index, 1);
        }
        // 如果没有剩余的 hotkey，注销后端快捷键
        if (hotkeys.length === 0) {
          await invoke('unregister_shortcut', { key: hotkey.key });
        }
      }
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  /**
   * 注销所有快捷键
   */
  async unregisterAll(): Promise<void> {
    try {
      // 调用后端注销所有快捷键
      await invoke('unregister_all_shortcuts');
      // 清空前端注册表
      shortcuts.current = {};
      console.debug('注销所有快捷键成功');
    } catch (error) {
      console.error('注销所有快捷键失败:', error);
      throw error;
    }
  }

  /**
   * 显示窗口
   */
  async showWindow(): Promise<void> {
    try {
      await invoke('show_window');
    } catch (error) {
      console.error('显示窗口失败:', error);
    }
  }
}

// 导出单例实例
export const manager = new Manager();
