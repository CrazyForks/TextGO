<script lang="ts">
  import { getKeyDisplay } from '$lib/helpers';

  const { shortcut, class: _class }: { shortcut: string; class?: string } = $props();

  // split shortcut into individual keys
  const keys = $derived(shortcut.split('+'));
</script>

<div class="flex items-center gap-0.5 text-primary/80 {_class}">
  {#each keys as code, index (code)}
    {@const key = getKeyDisplay(code)}
    {@const modifier = index < keys.length - 1}
    {#if modifier}
      <kbd class="kbd min-w-8 text-lg">{key}</kbd>
      <span class="text-sm opacity-30">+</span>
    {:else}
      <kbd class="kbd min-w-8">{key}</kbd>
    {/if}
  {/each}
</div>
