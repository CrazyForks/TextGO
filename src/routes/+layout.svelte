<script lang="ts">
  import { dev } from '$app/environment';
  import { Alert, Confirm } from '$lib/components';
  import { m } from '$lib/paraglide/messages';
  import { invoke } from '@tauri-apps/api/core';
  import type { Snippet } from 'svelte';
  import { onMount } from 'svelte';
  // import fonts
  import '@fontsource-variable/noto-sans';
  import '@fontsource-variable/noto-sans-sc';
  // import styles
  import 'tippy.js/animations/scale.css';
  import 'tippy.js/dist/tippy.css';
  import '../app.css';

  let { children }: { children: Snippet } = $props();

  // initialize tray menu language
  onMount(async () => {
    try {
      await invoke('setup_tray', {
        mainWindowText: m.tray_main_window(),
        shortcutsText: m.tray_shortcuts(),
        aboutText: m.tray_about(),
        quitText: m.tray_quit()
      });
    } catch (error) {
      console.error(`Failed to initialize tray menu language: ${error}`);
    }
  });

  // disable right-click menu
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

<!-- global alert component -->
<Alert />

<!-- global confirm component -->
<Confirm />
