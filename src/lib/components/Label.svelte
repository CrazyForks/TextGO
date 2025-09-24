<script lang="ts" module>
  import type { IconComponentProps } from 'phosphor-svelte';
  import type { Component, Snippet } from 'svelte';
  import type { Placement } from 'tippy.js';

  export type LabelProps = {
    /** 标签文本片段 */
    children: Snippet;
    /** 标签图标 */
    icon?: Component<IconComponentProps>;
    /** 提示文本 */
    tip?: string;
    /** 提示位置 */
    tipPlacement?: Placement;
    /** 是否标记必填 */
    required?: boolean;
    /** 自定义样式类名 */
    class?: string;
  };
</script>

<script lang="ts">
  import { tooltip } from '$lib/utils';
  import { Question } from 'phosphor-svelte';

  let { children, icon, tip, tipPlacement = 'left', required = false, class: _class }: LabelProps = $props();
</script>

<div class="flex items-center justify-between gap-2 p-1 {_class}">
  <span class="flex items-center gap-1">
    {#if required}
      <span class="h-6 text-lg text-error">*</span>
    {/if}
    {#if icon}
      {@const Icon = icon}
      <Icon class="mr-1 size-5" />
    {/if}
    <span class="text-base tracking-wide opacity-90">
      {@render children()}
    </span>
  </span>
  {#if tip}
    <span class="cursor-help" use:tooltip={{ content: tip, placement: tipPlacement }}>
      <Question class="size-5 opacity-60" />
    </span>
  {/if}
</div>
