<script lang="ts" module>
  import { alert } from '$lib/components';
  import { execute } from '$lib/executor';
  import { match } from '$lib/matcher';
  import { shortcuts } from '$lib/states.svelte';
  import type { Hotkey, Model, Option, Prompt, Script } from '$lib/types';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  /**
   * 快捷键管理类
   */
  export class HotkeyManager {
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
  export const hotkeyManager = new HotkeyManager();
</script>

<script lang="ts">
  import { enhance } from '$app/forms';
  import { Label, Modal, Select } from '$lib/components';
  import { MODEL_MARK, PROMPT_MARK, SCRIPT_MARK } from '$lib/constants';
  import { BUILTIN_CASES, NATURAL_CASES, PROGRAM_CASES } from '$lib/matcher';
  import { Loading } from '$lib/states.svelte';
  import { ArrowFatLineRight, FingerprintSimple, Sparkle } from 'phosphor-svelte';

  let {
    models,
    scripts,
    prompts
  }: {
    models: Model[];
    scripts: Script[];
    prompts: Prompt[];
  } = $props();

  // 加载状态
  const loading = new Loading();

  // 文本类型选项
  const textCases: Option[] = $derived.by(() => {
    const options: Option[] = [
      { value: '', label: '跳过' },
      { value: '--builtin--', label: '-- 常规 --', disabled: true },
      ...BUILTIN_CASES,
      { value: '--natural--', label: '-- 自然语言 --', disabled: true },
      ...NATURAL_CASES,
      { value: '--program--', label: '-- 编程语言 --', disabled: true },
      ...PROGRAM_CASES
    ];
    if (models && models.length > 0) {
      options.push({ value: '--model--', label: '-- 分类模型 --', disabled: true });
      for (const m of models) {
        options.push({ value: MODEL_MARK + m.id, label: m.id });
      }
    }
    return options;
  });

  // 动作标识选项
  const actionIds: Option[] = $derived.by(() => {
    const options: Option[] = [{ value: '', label: '显示主窗口' }];
    if (scripts && scripts.length > 0) {
      options.push({ value: '--script--', label: '-- 脚本 --', disabled: true });
      for (const s of scripts) {
        options.push({ value: SCRIPT_MARK + s.id, label: s.id });
      }
    }
    if (prompts && prompts.length > 0) {
      options.push({ value: '--prompt--', label: '-- 对话 --', disabled: true });
      for (const p of prompts) {
        options.push({ value: PROMPT_MARK + p.id, label: p.id });
      }
    }
    return options;
  });

  /**
   * 获取文本类型标签
   *
   * @param value - 文本类型值
   * @returns 文本类型标签
   */
  export function getCaseLabel(value: string): string | null {
    const option = textCases.find((c) => c.value === value);
    return option ? option.label : null;
  }

  /**
   * 获取动作标识标签
   *
   * @param value - 动作标识值
   * @returns 动作标识标签
   */
  export function getActionLabel(value: string): string | null {
    const option = actionIds.find((a) => a.value === value);
    return option ? option.label : null;
  }

  // 快捷键新增模态框
  let hotkeyModal: Modal;
  export const showModal = (key: string) => {
    lastKey = key.toUpperCase();
    hotkeyModal.show();
  };

  let lastKey: string = $state('');
  let textCase: string = $state('');
  let actionId: string = $state('');

  /**
   * 保存并注册快捷键
   *
   * @param form - 表单元素
   */
  async function register(form: HTMLFormElement) {
    const hotkeys = shortcuts.current[lastKey];
    if (hotkeys.find((h) => h.key === lastKey && h.case === textCase)) {
      alert({ level: 'error', message: '该快捷键组合已被使用' });
      return;
    }
    loading.start();
    try {
      const hotkey: Hotkey = {
        id: lastKey + textCase,
        key: lastKey,
        case: textCase,
        action: actionId
      };

      // 注册快捷键
      await hotkeyManager.register(hotkey);

      form.reset();
      hotkeyModal.close();
      alert('快捷键注册成功');
    } catch (error) {
      console.error('注册快捷键失败:', error);
      alert({ level: 'error', message: `注册快捷键失败: ${error}` });
    } finally {
      loading.end();
    }
  }

  /**
   * 注销并删除快捷键
   *
   * @param hotkey - 快捷键配置
   */
  export async function unregister(hotkey: Hotkey) {
    try {
      await hotkeyManager.unregister(hotkey);
    } catch (error) {
      console.error('注销快捷键失败:', error);
      alert({ level: 'error', message: `注销快捷键失败: ${error}` });
    }
  }
</script>

<Modal icon={Sparkle} title="新增规则" bind:this={hotkeyModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      register(formElement);
    }}
  >
    <fieldset class="fieldset">
      <Label icon={FingerprintSimple} class="mt-4">识别</Label>
      <Select bind:value={textCase} options={textCases} class="w-full" />
      <Label icon={ArrowFatLineRight} class="mt-4">执行</Label>
      <Select bind:value={actionId} options={actionIds} class="w-full" />
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => hotkeyModal?.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        确 定
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
