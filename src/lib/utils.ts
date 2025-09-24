import type { ActionReturn } from 'svelte/action';
import type { Instance, Props } from 'tippy.js';
import tippy from 'tippy.js';

/**
 * 防抖函数
 *
 * @param func - 需要防抖的函数
 * @param timeout - 防抖延迟的毫秒数
 * @param immediate - 是否立即触发函数
 * @returns 防抖后的函数
 */
export function debounce<F extends (...args: Parameters<F>) => ReturnType<F>>(
  func: F,
  timeout: number = 500,
  immediate: boolean = false
): (...args: Parameters<F>) => Promise<ReturnType<F> | undefined> {
  let timeoutId: ReturnType<typeof setTimeout> | null;
  return (...args: Parameters<F>) => {
    return new Promise((resolve, reject) => {
      if (timeoutId) {
        clearTimeout(timeoutId);
        if (immediate) {
          // 如果被防抖则返回 undefined
          resolve(undefined);
        }
      } else if (immediate) {
        try {
          resolve(func(...args));
        } catch (error) {
          reject(error);
        }
      }
      timeoutId = setTimeout(() => {
        if (!immediate) {
          try {
            resolve(func(...args));
          } catch (error) {
            reject(error);
          }
        }
        timeoutId = null;
      }, timeout);
    });
  };
}

/**
 * 节流函数
 *
 * @param func - 需要节流的函数
 * @param timeout - 节流延迟的毫秒数
 * @param immediate - 是否立即触发函数
 * @returns 节流后的函数
 */
export function throttle<F extends (...args: Parameters<F>) => ReturnType<F>>(
  func: F,
  timeout: number = 500,
  immediate: boolean = true
): (...args: Parameters<F>) => Promise<ReturnType<F> | undefined> {
  let previous = 0;
  return (...args: Parameters<F>) => {
    return new Promise((resolve, reject) => {
      const now = Date.now();
      if (now - previous > timeout) {
        previous = now;
        if (immediate) {
          try {
            resolve(func(...args));
          } catch (error) {
            reject(error);
          }
        } else {
          setTimeout(() => {
            try {
              resolve(func(...args));
            } catch (error) {
              reject(error);
            }
          }, timeout);
        }
      } else {
        // 如果被节流则返回 undefined
        resolve(undefined);
      }
    });
  };
}

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
 * 转义字符串中的 HTML 特殊字符以防止 XSS 攻击
 *
 * @param str - 需要转义的字符串
 * @returns 转义后的字符串
 */
export function escapeHTML(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

/**
 * ISO8601 格式日期时间字符串转 yyyy-MM-dd HH:mm:ss
 *
 * @param str - ISO8601 格式日期时间字符串
 * @returns 格式化后的日期时间字符串
 */
export function formatISO8601(str: string | null | undefined): string {
  if (!str) {
    return '';
  }
  const d = new Date(str);
  const p = (n: number) => String(n).padStart(2, '0');
  return (
    `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}` +
    ` ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`
  );
}
