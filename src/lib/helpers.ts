import { messages } from '$lib/components/Confirm.svelte';
import { modals } from '$lib/components/Modal.svelte';
import { getLocale } from '$lib/paraglide/runtime';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { ActionReturn } from 'svelte/action';
import type { Instance, Props } from 'tippy.js';
import tippy from 'tippy.js';

/**
 * 使用 Tippy.js 创建工具提示
 *
 * @param target - 目标元素
 * @param props - 提示属性
 * @returns svelte action 返回值
 */
export function tooltip(target: HTMLElement, props: Partial<Props>): ActionReturn<Partial<Props>> {
  let instance: Instance | null = null;
  if (props && props.content) {
    // 检查目标元素是否在 dialog 元素内
    let el: HTMLElement | null = target;
    while (el && el.nodeName !== 'DIALOG') {
      el = el.parentElement;
    }
    const dialog = el as HTMLDialogElement | null;
    if (dialog) {
      // 将 appendTo 属性设置为 dialog 元素
      props.appendTo = dialog;
    }
    // 创建工具提示实例
    instance = tippy(target, props);
  }
  return {
    update: (props) => {
      if (props && props.content) {
        if (instance) {
          instance.setProps(props);
        } else {
          instance = tippy(target, props);
        }
      } else if (instance) {
        instance.destroy();
        instance = null;
      }
    },
    destroy: () => {
      if (instance) {
        instance.destroy();
      }
    }
  };
}

/**
 * 格式化 ISO8601 日期时间字符串
 *
 * @param str - ISO8601 格式日期时间字符串
 * @returns 格式化后的日期时间字符串
 */
export function formatISO8601(str: string | null | undefined): string {
  if (!str) {
    return '';
  }
  const datetime = new Date(str);
  return datetime.toLocaleString(getLocale(), {
    dateStyle: 'medium',
    timeStyle: 'medium'
  });
}

/**
 * 冻结当前窗口大小，防止快速调整窗口大小时边缘出现空白
 * https://github.com/tauri-apps/tauri/issues/6322
 */
export function freeze(): Promise<void> {
  return getCurrentWindow().setResizable(false);
}

/**
 * 当前没有消息框和模态框时解冻窗口大小
 */
export function unfreeze(): Promise<void> {
  if (messages.size === 0 && modals.size === 0) {
    return getCurrentWindow().setResizable(true);
  }
  return Promise.reject();
}
