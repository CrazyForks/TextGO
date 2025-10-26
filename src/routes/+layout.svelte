<script lang="ts">
  import { dev } from '$app/environment';
  import { Alert, Confirm } from '$lib/components';
  import { m } from '$lib/paraglide/messages';
  import { invoke } from '@tauri-apps/api/core';
  import type { Snippet } from 'svelte';
  import { onMount } from 'svelte';
  // 导入字体
  import '@fontsource-variable/noto-sans';
  import '@fontsource-variable/noto-sans-sc';
  // 导入样式
  import 'tippy.js/animations/scale.css';
  import 'tippy.js/dist/tippy.css';
  import '../app.css';

  let { children }: { children: Snippet } = $props();

  // 初始化托盘菜单语言
  onMount(async () => {
    try {
      await invoke('update_tray_menu', {
        toggleText: m.tray_toggle(),
        shortcutsText: m.tray_shortcuts(),
        aboutText: m.tray_about(),
        quitText: m.tray_quit()
      });
    } catch (error) {
      console.error('初始化托盘菜单语言失败:', error);
    }
  });

  // 禁用右键菜单
  if (!dev) {
    onMount(() => {
      const disableContextMenu = (event: MouseEvent) => {
        event.preventDefault();
        return false;
      };
      document.addEventListener('contextmenu', disableContextMenu);
      return () => {
        document.removeEventListener('contextmenu', disableContextMenu);
      };
    });
  }
</script>

{@render children()}

<!-- 全局提示组件 -->
<Alert />

<!-- 全局确认组件 -->
<Confirm />
