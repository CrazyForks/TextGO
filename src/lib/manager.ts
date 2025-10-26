import { execute } from '$lib/executor';
import { match } from '$lib/matcher';
import { shortcuts } from '$lib/stores.svelte';
import type { Rule } from '$lib/types';
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
      // 获取所有绑定到该键位的规则
      const rules = shortcuts.current[key];
      if (!rules || rules.length === 0) {
        return;
      }
      // 匹配要执行的动作
      const rule = await match(selection, rules);
      if (rule === null) {
        console.warn('没有匹配的规则');
        return;
      }
      // 执行默认动作
      if (rule.action === '') {
        await invoke('show_window');
        return;
      }
      // 执行动作
      await execute(rule, selection);
    } catch (error) {
      console.error('处理快捷键事件失败:', error);
    }
  }

  /**
   * 注册规则
   *
   * @param rule - 规则对象
   */
  async register(rule: Rule): Promise<void> {
    try {
      // 检查后端快捷键是否注册
      const isRegistered = await invoke('is_shortcut_registered', { key: rule.key });
      if (!isRegistered) {
        // 注册后端快捷键
        await invoke('register_shortcut', { key: rule.key });
      }
      // 保存规则到前端注册表中
      const rules = shortcuts.current[rule.key];
      if (rules && !rules.find((r) => r.id === rule.id)) {
        rules.push(rule);
      }
    } catch (error) {
      console.error('注册规则失败:', error);
      throw error;
    }
  }

  /**
   * 注销规则
   *
   * @param rule - 规则对象
   */
  async unregister(rule: Rule): Promise<void> {
    try {
      // 从前端注册表中移除规则
      const rules = shortcuts.current[rule.key];
      if (rules) {
        const index = rules.findIndex((r) => r.id === rule.id);
        if (index !== -1) {
          rules.splice(index, 1);
        }
        // 没有剩余规则时注销后端快捷键
        if (rules.length === 0) {
          await invoke('unregister_shortcut', { key: rule.key });
        }
      }
    } catch (error) {
      console.error('注销规则失败:', error);
      throw error;
    }
  }
}

// 导出单例实例
export const manager = new Manager();
