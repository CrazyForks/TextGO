import { logs, models, prompts, scripts, shortcuts, theme } from '$lib/states.svelte';
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
    zIndex: 98,
    maxWidth: 300,
    arrow: false,
    theme: 'neutral',
    animation: 'scale',
    placement: 'bottom',
    trigger: 'mouseenter',
    plugins: [followCursor]
  });

  return { theme, shortcuts, models, scripts, prompts, logs };
};
