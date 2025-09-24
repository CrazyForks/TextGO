<script lang="ts" module>
  import type { IconComponentProps } from 'phosphor-svelte';
  import type { Component, Snippet } from 'svelte';
  import type { MouseEventHandler } from 'svelte/elements';

  export type ButtonProps = Partial<{
    /** 按钮内容片段 */
    children: Snippet;
    /** 按钮图标 */
    icon: Component<IconComponentProps>;
    /** 图标粗细 */
    weight: 'thin' | 'light' | 'regular' | 'bold' | 'fill' | 'duotone';
    /** 按钮文本 */
    text: string;
    /** 按钮尺寸 */
    size: keyof typeof PRESETS;
    /** 是否使用轻按钮样式 */
    subtle: boolean;
    /** 是否使按钮为正方形 */
    square: boolean;
    /** 是否为按钮添加边框 */
    border: boolean;
    /** 是否为按钮添加阴影 */
    shadow: boolean;
    /** 是否显示加载动画 */
    loading: boolean | null;
    /** 是否禁用按钮 */
    disabled: boolean;
    /** 自定义样式类名 */
    class: string;
    iconClass: string;
    textClass: string;
    /** 按钮点击事件回调 */
    onclick: MouseEventHandler<HTMLButtonElement>;
  }>;

  /**
   * 按钮的预设尺寸
   */
  const PRESETS = {
    xs: {
      size: '1.5rem',
      iconSize: '1.125rem',
      textClass: 'text-xs'
    },
    sm: {
      size: '1.75rem',
      iconSize: '1.25rem',
      textClass: 'text-sm'
    }
  };
</script>

<script lang="ts">
  import { tooltip } from '$lib/utils';

  let {
    children,
    icon,
    weight = 'regular',
    text,
    size = 'xs',
    subtle = true,
    square = true,
    border = false,
    shadow = false,
    loading = false,
    disabled = false,
    class: _class,
    iconClass,
    textClass,
    onclick
  }: ButtonProps = $props();

  const preset = PRESETS[size];

  let subtleClass = $derived(subtle ? 'btn-subtle' : '');
  let squareClass = $derived(square ? 'p-0' : 'px-2');
  let borderClass = $derived(border ? 'border' : '');
  let shadowClass = $derived(shadow && !disabled ? 'shadow-sm' : '');
</script>

<button
  type="button"
  class="btn truncate opacity-80 {subtleClass} {squareClass} {borderClass} {shadowClass} {_class}"
  style:width={square ? preset.size : ''}
  style:height={preset.size}
  style:min-height={preset.size}
  {disabled}
  onclick={(event) => {
    !disabled && !loading && onclick?.(event);
  }}
  use:tooltip={{
    content: square ? text : '',
    followCursor: true
  }}
>
  {#if children}
    {@render children()}
  {:else}
    {#if loading}
      <span class="loading loading-spinner" style:width={preset.iconSize}></span>
    {:else}
      {@const Icon = icon}
      <Icon size={preset.iconSize} {weight} class={iconClass} />
    {/if}
    <span class="truncate {square ? 'sr-only' : preset.textClass} {textClass}" title={text}>{text}</span>
  {/if}
</button>
