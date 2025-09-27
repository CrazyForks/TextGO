<script lang="ts" module>
  import type { IconComponentProps } from 'phosphor-svelte';
  import type { Component, Snippet } from 'svelte';

  export type SettingProps = {
    /** 设置内容片段 */
    children: Snippet;
    /** 设置图标 */
    icon?: Component<IconComponentProps>;
    /** 设置标题 */
    title: string;
    /** 提示文本 */
    tip?: string;
    /** 自定义样式类名 */
    class?: string;
  };
</script>

<script lang="ts">
  import { tooltip } from '$lib/utils';
  import { Question } from 'phosphor-svelte';

  let { children, icon, title, tip, class: _class }: SettingProps = $props();
</script>

<div class="rounded-container {_class}">
  <div class="flex items-center justify-between">
    <span class="flex items-center gap-2">
      {#if icon}
        {@const Icon = icon}
        <Icon class="size-6 select-none" />
      {/if}
      <span class="text-lg font-semibold tracking-wider">
        {title}
      </span>
    </span>
    {#if tip}
      <span class="cursor-help" use:tooltip={{ content: tip, placement: 'left' }}>
        <Question class="size-5 opacity-60" />
      </span>
    {/if}
  </div>
  <div class="divider my-1"></div>
  <div class="flex flex-col gap-4">
    {@render children()}
  </div>
</div>
