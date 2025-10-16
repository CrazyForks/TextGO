import { execute } from '$lib/executor';
import { match } from '$lib/matcher';
import { shortcuts } from '$lib/states.svelte';
import type { Hotkey } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

/**
 * 快捷键管理类
 */
export class Manager {
  constructor() {
    this.initialize();
  }

  /**
   * 初始化事件监听器
   */
  private async initialize(): Promise<void> {
    if (getCurrentWindow().label === 'main') {
      try {
        // 监听来自 Rust 后端的快捷键触发事件
        await listen('shortcut-triggered', async (event) => {
          const payload = event.payload as { key: string; selection: string };
          await this.handleShortcutEvent(payload.key, payload.selection);
        });
      } catch (error) {
        console.error('初始化快捷键事件监听器失败:', error);
      }
    }
  }

  /**
   * 处理快捷键事件
   *
   * @param key - 触发的快捷键
   * @param selection - 选中的文本
   */
  private async handleShortcutEvent(key: string, selection: string): Promise<void> {
    try {
      // 获取所有绑定到该键位的快捷键
      const hotkeys = shortcuts.current[key];
      if (!hotkeys || hotkeys.length === 0) {
        return;
      }
      // 匹配要执行的动作
      const hotkey = await match(selection, hotkeys);
      if (hotkey === null) {
        console.warn('没有匹配的快捷键动作');
        return;
      }
      // 执行默认动作
      if (hotkey.action === '') {
        await invoke('show_window');
        return;
      }
      // 执行动作
      await execute(hotkey, selection);
    } catch (error) {
      console.error('处理快捷键事件失败:', error);
    }
  }

  /**
   * 注册全局快捷键
   *
   * @param hotkey - 快捷键配置
   */
  async register(hotkey: Hotkey): Promise<void> {
    try {
      // 检查后端 shortcut 是否注册
      const isRegistered = await invoke('is_shortcut_registered', { key: hotkey.key });
      if (!isRegistered) {
        // 注册后端 shortcut
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
   * 注销全局快捷键
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
        // 没有剩余的 hotkey 时注销后端 shortcut
        if (hotkeys.length === 0) {
          await invoke('unregister_shortcut', { key: hotkey.key });
        }
      }
    } catch (error) {
      console.error(error);
      throw error;
    }
  }
}

// 导出单例实例
export const manager = new Manager();
