<script lang="ts" module>
  import type { Snippet } from 'svelte';

  export type TitleProps = {
    /** 窗口标题片段 */
    children?: Snippet;
    /** 默认标题片段 */
    fallback?: Snippet;
    /** 自定义样式类名 */
    class?: string;
  };

  let title: Snippet | null = $state(null);
</script>

<script lang="ts">
  import { onMount } from 'svelte';

  let { children, fallback, class: _class }: TitleProps = $props();

  onMount(() => {
    if (children) {
      title = children;
      return () => {
        title = null;
      };
    }
  });
</script>

{#if !children}
  <div class="flex h-11 shrink-0 items-center justify-center select-none {_class}" data-tauri-drag-region>
    {#if title}
      {@render title()}
    {:else if fallback}
      {@render fallback()}
    {/if}
  </div>
{/if}
