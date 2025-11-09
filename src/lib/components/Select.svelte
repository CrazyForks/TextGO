<script lang="ts" module>
  import type { Option, OptionValue } from '$lib/types';
  import type { Snippet } from 'svelte';
  import type { ChangeEventHandler } from 'svelte/elements';

  export type SelectProps = Partial<{
    /**
     * Option content snippet
     */
    children: Snippet;
    /**
     * Option list
     */
    options: Option[];
    /**
     * Selected value
     */
    value: OptionValue;
    /**
     * Placeholder text
     */
    placeholder: string;
    /**
     * Whether to disable select box
     */
    disabled: boolean;
    /**
     * Custom style class name
     */
    class: string;
    /**
     * Callback function when selected value changes
     */
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

  // options to render
  let options = $derived.by(() => {
    if (placeholder) {
      return [{ value: null, label: placeholder, disabled: true }, ...(_options || [])];
    } else {
      return _options;
    }
  });

  // null value is used for placeholder
  // empty string is used for all options
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
