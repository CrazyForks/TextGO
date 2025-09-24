<script lang="ts">
  import { type } from '@tauri-apps/plugin-os';
  import { ArrowFatUp, Command, Control } from 'phosphor-svelte';

  const { key, class: _class }: { key?: string; class?: string } = $props();
  const osType = type();
</script>

<div class="flex items-center gap-1 text-active {_class}">
  <kbd class="kbd px-1.5 kbd-sm">
    {#if osType === 'macos'}
      <Command class="size-[14px]" />
    {:else}
      <Control class="size-[14px]" />
    {/if}
  </kbd>
  <kbd class="kbd px-1.5 kbd-sm">
    <ArrowFatUp class="size-[14px]" />
  </kbd>
  <kbd class="kbd px-2 kbd-sm">
    <div class="text-sm font-light">{key}</div>
  </kbd>
</div>
