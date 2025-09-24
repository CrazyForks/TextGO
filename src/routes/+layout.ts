import * as stores from '$lib/stores.svelte';
import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';
import tippy, { followCursor } from 'tippy.js';
import type { LayoutLoad } from './$types';

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export const load: LayoutLoad = async () => {
  // 设置 Tippy.js 全局默认属性
  // https://atomiks.github.io/tippyjs/v6/methods/#setdefaultprops
  tippy.setDefaultProps({
    zIndex: 100,
    maxWidth: 300,
    arrow: false,
    theme: 'default',
    animation: 'scale',
    placement: 'bottom',
    trigger: 'mouseenter',
    plugins: [followCursor]
  });

  // 转发 console 日志到 Tauri 日志插件
  forwardConsole('log', trace);
  forwardConsole('debug', debug);
  forwardConsole('info', info);
  forwardConsole('warn', warn);
  forwardConsole('error', error);

  // 导入时立即初始化所有 stores
  return { stores };
};

/**
 * 转发 console 方法到指定的日志记录器
 *
 * @param fnName - console 方法名称
 * @param logger - 日志记录器函数
 */
function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}
