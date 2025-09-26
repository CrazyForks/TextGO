import { hotkeyManager } from '$lib/components/Hotkey.svelte';
import { LOGS_KEY, MODELS_KEY, PROMPTS_KEY, SCRIPTS_KEY, SHORTCUTS_KEY } from '$lib/constants';
import type { Hotkey, Log, Model, Prompt, Script } from '$lib/types';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LazyStore } from '@tauri-apps/plugin-store';
import { untrack } from 'svelte';

// 创建一个全局的 LazyStore 实例
const store = new LazyStore('.settings.dat');

// 主题存储
export const theme = persisted<string>('theme', 'light', {
  onchange: (theme) => {
    // 动态设置根元素的 data-theme 属性以应用主题
    const root = document.documentElement;
    root.setAttribute('data-theme', theme);
    // 此处设置的主题是应用范围的，而不是特定于当前窗口
    getCurrentWindow().setTheme(theme === 'light' ? 'light' : 'dark');
  }
});

// 快捷键组存储
export const shortcuts = persisted<Record<string, Hotkey[]>>(
  SHORTCUTS_KEY,
  {},
  {
    onload: async (shortcuts) => {
      // 主窗口初始化时注册所有快捷键
      if (getCurrentWindow().label === 'main') {
        for (const hotkey of Object.values(shortcuts).flat()) {
          await hotkeyManager.register(hotkey);
        }
      }
    }
  }
);

// 分类模型存储
export const models = persisted<Model[]>(MODELS_KEY, []);

// 脚本存储
export const scripts = persisted<Script[]>(SCRIPTS_KEY, []);

// 提示词存储
export const prompts = persisted<Prompt[]>(PROMPTS_KEY, []);

// 触发记录存储
export const logs = persisted<Log[]>(LOGS_KEY, []);

/**
 * 创建持久化状态的选项
 */
type Options<T> = {
  /** 加载完成时的回调函数 */
  onload?: (value: T) => void;
  /** 存储值变化时的回调函数 */
  onchange?: (value: T | $state.Snapshot<T>) => void;
};

/**
 * 创建一个持久化的响应式状态
 *
 * @param key - 本地存储的键
 * @param initial - 初始值
 * @param onchange - 存储值变化时的回调函数
 * @returns 持久化状态对象
 */
export function persisted<T>(key: string, initial: T, options?: Options<T>) {
  let state = $state(initial);
  let initialized = $state(false);

  // 从本地存储加载数据
  store
    .get<T>(key)
    .then((item) => {
      if (item !== undefined) {
        untrack(() => {
          state = item;
        });
        options?.onload?.(item);
        options?.onchange?.(item);
      }
      initialized = true;
    })
    .catch((error) => {
      console.error(error);
      initialized = true;
    });

  // 保存数据到本地存储
  $effect.root(() => {
    $effect(() => {
      if (!initialized) {
        return;
      }
      // 获取当前状态的快照
      const snapshot = $state.snapshot(state);
      store
        .set(key, snapshot)
        .then(() => {
          // 保存到本地文件
          store.save();
        })
        .then(() => {
          options?.onchange?.(snapshot);
        })
        .catch((error) => {
          console.error(error);
        });

      if (snapshot) {
        localStorage.setItem(key, JSON.stringify(snapshot));
      }
    });
    // 确保不被清理
    return () => {};
  });

  window.addEventListener('storage', (event) => {
    if (event.key === key && event.newValue) {
      state = JSON.parse(event.newValue);
    }
  });

  return {
    get current() {
      return state;
    },
    set current(value: T) {
      state = value;
    }
  };
}

/**
 * 获取持久化的状态数据
 *
 * @param key - 本地存储的键
 * @param defaultValue - 如果键不存在时的默认值
 * @returns 返回获取到的值或默认值
 */
export async function getPersisted<T>(key: string, defaultValue?: T): Promise<T | undefined> {
  try {
    const value = await store.get<T>(key);
    return value !== undefined ? value : defaultValue;
  } catch (error) {
    console.error(error);
    return defaultValue;
  }
}

/**
 * 表示加载状态的响应式类
 */
export class Loading {
  private loading = $state<boolean | null>(null);

  /**
   * 将加载状态设置为`false`，然后在指定的延迟后将其设置为`true`
   *
   * @param delay - 在将加载状态设置为`true`之前等待的毫秒数
   */
  start(delay: number = 500) {
    this.loading = false;
    if (delay <= 0) {
      this.loading = true;
    } else {
      setTimeout(() => {
        if (this.loading === false) this.loading = true;
      }, delay);
    }
  }

  /**
   * 将加载状态设置为`null`，表示加载结束
   */
  end() {
    this.loading = null;
  }

  /**
   * 获取当前加载状态
   */
  get current() {
    return this.loading;
  }

  /**
   * 是否已经开始加载
   */
  get started() {
    return this.loading !== null;
  }

  /**
   * 是否已经达到延迟时间
   */
  get delayed() {
    return this.loading === true;
  }
}
