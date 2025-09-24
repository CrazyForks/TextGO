<script lang="ts" module>
  import type { IconComponentProps } from 'phosphor-svelte';
  import type { Component, Snippet } from 'svelte';
  import { SvelteMap } from 'svelte/reactivity';

  export type ModalProps = Partial<{
    /** 标题前显示的图标 */
    icon: Component<IconComponentProps>;
    /** 模态对话框的标题 */
    title: string;
    /** 模态对话框的内容 */
    children: Snippet;
    /** 模态对话框的最大宽度 */
    maxWidth: string;
    /** 模态对话框的类名 */
    class: string;
    boxClass: string;
    cornerClass: string;
    /** 对话框关闭时的回调函数 */
    onclose: () => void;
  }>;

  /**
   * 模态对话框的响应式映射
   */
  export const modals = new SvelteMap<string, ModalProps>();
</script>

<script lang="ts">
  import { Alert } from '$lib/components';
  import { freeze, unfreeze } from '$lib/helpers';
  import { tick } from 'svelte';
  import { fade } from 'svelte/transition';

  let { icon, title, children, maxWidth, class: _class, boxClass, cornerClass, onclose }: ModalProps = $props();
  let dialog: HTMLDialogElement | null = $state(null);
  const id: string = `modal-${crypto.randomUUID()}`;

  /**
   * 显示模态对话框
   */
  export function show() {
    modals.set(id, { onclose });
    tick().then(() => {
      if (!dialog) {
        return;
      }
      dialog.showModal();
      // 设置焦点
      const autofocus = dialog?.querySelector('.autofocus') as HTMLElement | null;
      if (autofocus) {
        setTimeout(() => autofocus.focus(), 0);
      }
      // 冻结窗口
      freeze();
    });
  }

  /**
   * 关闭模态对话框
   *
   * @param event - 触发关闭操作的鼠标事件
   */
  export function close(event: MouseEvent | null = null) {
    if (event) {
      event.preventDefault();
    }
    let modal = modals.get(id);
    if (modal) {
      modal.onclose?.();
      modals.delete(id);
      unfreeze();
    }
  }

  /**
   * 检查模态对话框当前是否打开
   *
   * @return 如果模态对话框打开则返回 true，否则返回 false
   */
  export function isOpen(): boolean {
    return modals.has(id);
  }
</script>

{#if modals.has(id)}
  <dialog {id} class="modal transition-none {_class}" bind:this={dialog} transition:fade={{ duration: 200 }}>
    <form method="dialog" class="modal-backdrop">
      <button onclick={close} aria-label="Close"></button>
    </form>
    <div class="modal-box {boxClass}" style:max-width={maxWidth}>
      <form method="dialog" class="modal-corner {cornerClass}">
        <button onclick={close}>✕</button>
      </form>
      {#if title}
        <h3 class="modal-title">
          {#if icon}
            {@const Icon = icon}
            <Icon class="size-6" />
          {/if}
          {title}
        </h3>
      {/if}
      {#if children}
        {@render children()}
      {/if}
    </div>
    <Alert dialog />
  </dialog>
{/if}
