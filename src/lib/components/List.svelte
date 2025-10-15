<script lang="ts" generics="T extends { id: string }">
  import { Button, confirm } from '$lib/components';
  import {
    ArrowCircleDown,
    ArrowCircleUp,
    DotsThree,
    Lightbulb,
    PlusCircle,
    XCircle,
    type IconComponentProps
  } from 'phosphor-svelte';
  import type { Component, Snippet } from 'svelte';
  import { flip } from 'svelte/animate';

  type ListProps = {
    /** 列表标题 */
    title?: string | Snippet;
    /** 标题图标 */
    icon?: Component<IconComponentProps>;
    /** 提示文本 */
    hint?: string;
    /** 数据名称 */
    name?: string;
    /** 列表数据 */
    data: T[];
    /** 数据行片段 */
    row: Snippet<[T]>;
    /** 自定义样式类名 */
    class?: string;
    /** 点击创建时的回调函数 */
    oncreate?: () => void;
    /** 数据删除后的回调函数 */
    ondelete?: (item: T) => void;
    /** 点击更多操作时的回调函数 */
    moreActions?: () => void;
  };

  let {
    title = '',
    icon,
    hint = '',
    name = '',
    data = $bindable(),
    row,
    class: _class,
    oncreate,
    ondelete,
    moreActions
  }: ListProps = $props();

  // 选中数据ID
  let selectedId: string = $state('');
  // 选中数据编号
  let selectedNum: string = $state('');
  // 选中数据元素
  let selectedElement: HTMLLIElement | null = $state(null);

  /**
   * 把选中的行滚动到可见区域
   */
  function scrollIntoView() {
    if (!selectedElement) {
      return;
    }
    // 等待FLIP动画完成后执行
    setTimeout(() => {
      selectedElement?.scrollIntoView({
        behavior: 'smooth',
        block: 'nearest',
        inline: 'nearest'
      });
    }, 200);
  }
</script>

<div class="overflow-hidden rounded-box border shadow-xs {_class}">
  <div class="flex items-center justify-between border-b gradient px-2 py-1">
    <span class="flex items-center gap-1 text-base-content/80">
      {#if icon}
        {@const Icon = icon}
        <Icon class="mx-1 size-4" />
      {/if}
      {#if typeof title === 'string'}
        <span class="text-sm tracking-wide">{title}</span>
      {:else}
        {@render title()}
      {/if}
    </span>
    <span class="flex items-center gap-1">
      <Button icon={PlusCircle} weight="bold" text="新增{name}" class="text-green-800" onclick={() => oncreate?.()} />
      <Button
        icon={XCircle}
        weight="bold"
        text="删除{name}"
        class={selectedId ? 'text-red-800' : 'btn-disabled'}
        onclick={() => {
          if (!selectedId) {
            return;
          }
          // 确认删除操作
          confirm({
            title: `删除${name}[${selectedNum}]`,
            message: '数据删除后无法恢复，是否继续？',
            onconfirm: () => {
              const index = data.findIndex((i) => i.id === selectedId);
              if (index !== -1) {
                const item = data[index];
                data.splice(index, 1);
                ondelete?.(item);
              }
              selectedId = '';
              selectedNum = '';
              selectedElement = null;
            }
          });
        }}
      />
      <Button
        icon={ArrowCircleUp}
        weight="bold"
        text="上移"
        class={selectedId ? 'text-surface' : 'btn-disabled'}
        onclick={() => {
          if (!selectedId) {
            return;
          }
          const index = data.findIndex((i) => i.id === selectedId);
          if (index > 0) {
            const temp = data[index];
            data[index] = data[index - 1];
            data[index - 1] = temp;
          }
          scrollIntoView();
        }}
      />
      <Button
        icon={ArrowCircleDown}
        weight="bold"
        text="下移"
        class={selectedId ? 'text-surface' : 'btn-disabled'}
        onclick={() => {
          if (!selectedId) {
            return;
          }
          const index = data.findIndex((i) => i.id === selectedId);
          if (index < data.length - 1) {
            const temp = data[index];
            data[index] = data[index + 1];
            data[index + 1] = temp;
          }
          scrollIntoView();
        }}
      />
      {#if moreActions}
        <Button
          icon={DotsThree}
          weight="bold"
          text="更多操作"
          onclick={() => {
            moreActions?.();
          }}
        />
      {/if}
    </span>
  </div>
  <ul class="list scrollbar-none overflow-y-auto bg-base-100 [&_.list-row]:min-h-10 [&_.list-row]:py-1">
    {#if data.length === 0}
      <li class="list-row mx-auto items-center gap-1 text-surface/35">
        <Lightbulb class="size-3.5" />{hint || '暂无数据'}
      </li>
    {/if}
    {#each data as item, index (item.id)}
      {@const itemNum = (index + 1).toString().padStart(2, '0')}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <li
        class="list-row cursor-pointer items-center rounded-none hover:bg-base-300"
        onclick={(event) => {
          if (selectedId === item.id) {
            selectedId = '';
            selectedNum = '';
            selectedElement = null;
          } else {
            selectedId = item.id;
            selectedNum = itemNum;
            selectedElement = event.currentTarget as HTMLLIElement;
          }
        }}
        animate:flip={{ duration: 200 }}
      >
        <span class="flex items-center gap-1">
          <input type="radio" class="pointer-events-none radio radio-xs" checked={selectedId === item.id} />
          <span class="text-base font-thin {selectedId === item.id ? '' : 'opacity-60'}">{itemNum}</span>
        </span>
        {@render row(item)}
      </li>
    {/each}
  </ul>
</div>
