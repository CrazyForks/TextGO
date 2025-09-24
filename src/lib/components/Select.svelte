<script lang="ts" module>
  import type { Option, OptionValue } from '$lib/types';
  import type { Snippet } from 'svelte';
  import type { ChangeEventHandler } from 'svelte/elements';

  export type SelectProps = Partial<{
    /** 选项内容片段 */
    children: Snippet;
    /** 选项列表 */
    options: Option[];
    /** 选中的值 */
    value: OptionValue;
    /** 占位符文本 */
    placeholder: string;
    /** 是否禁用选择框 */
    disabled: boolean;
    /** 自定义样式类名 */
    class: string;
    /** 选中值变化时的回调函数 */
    onchange: ChangeEventHandler<HTMLSelectElement>;
  }>;
</script>

<script lang="ts">
  let {
    children,
    options: _options,
    value = $bindable(),
    placeholder,
    disabled = false,
    class: _class,
    onchange
  }: SelectProps = $props();

  // 要渲染的选项
  let options = $derived.by(() => {
    if (placeholder) {
      return [{ value: null, label: placeholder, disabled: true }, ...(_options || [])];
    } else {
      return _options;
    }
  });

  // null 值用于表示占位符
  // 空字符串用于表示所有选项
  $effect(() => {
    if (placeholder && value === '') {
      value = null;
    }
  });
</script>

<select
  class="select appearance-none truncate {value ? '' : 'text-base-content/50'} {_class}"
  bind:value
  {disabled}
  {onchange}
>
  {#if children}
    {@render children()}
  {:else if options}
    {#each options as option (option.value)}
      <option value={option.value} disabled={option.disabled}>
        {option.label}
      </option>
    {/each}
  {/if}
</select>
