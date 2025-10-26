<script lang="ts" module>
  import { m } from '$lib/paraglide/messages';
  import { escapeHTML, freeze, unfreeze } from '$lib/utils';
  import { Warning, type IconComponentProps } from 'phosphor-svelte';
  import { tick, type Component } from 'svelte';
  import { SvelteMap } from 'svelte/reactivity';
  import { fade } from 'svelte/transition';

  export type Message = Partial<{
    /** 确认消息图标 */
    icon: Component<IconComponentProps>;
    /** 确认消息标题 */
    title: string;
    /** 确认消息内容 */
    message: string;
    /** 取消时的回调函数 */
    oncancel: () => void;
    /** 确认时的回调函数 */
    onconfirm: () => void;
  }>;

  /**
   * 确认消息的响应式映射
   */
  export const messages = new SvelteMap<string, Message>();

  /**
   * 显示确认消息对话框
   *
   * @param msg - 确认消息实例
   */
  export function confirm(msg: Message) {
    const id = `msg-${crypto.randomUUID()}`;
    messages.set(id, msg);
    // 等待对话框被添加到DOM中
    tick().then(() => {
      const dialog = document.getElementById(id) as HTMLDialogElement | null;
      if (!dialog) {
        return;
      }
      dialog.showModal();
      // 冻结窗口
      freeze();
    });
  }

  /**
   * 关闭确认消息对话框
   *
   * @param id - 消息对话框的ID
   * @param callback - 关闭前要调用的回调函数
   * @param event - 触发关闭操作的鼠标事件
   */
  function close(id: string, callback?: () => void, event: MouseEvent | null = null) {
    if (event) {
      event.preventDefault();
    }
    let msg = messages.get(id);
    if (msg) {
      callback?.();
      messages.delete(id);
      unfreeze();
    }
  }

  /**
   * 将标题的部分内容转换为特殊格式
   *
   * @param title - 标题字符串
   * @return 转换后的HTML字符串
   */
  function pretty(title: string): string {
    title = escapeHTML(title);
    return title.replace(/\[([^\]]+)\]/g, '<span class="truncate font-normal opacity-60">[$1]</span>');
  }
</script>

{#each messages.entries() as [id, msg] (id)}
  {@const Icon = msg.icon || Warning}
  <dialog {id} class="modal transition-none" transition:fade={{ duration: 200 }}>
    <form method="dialog" class="modal-backdrop">
      <button onclick={(event) => close(id, msg.oncancel, event)} aria-label="Close"></button>
    </form>
    <div class="modal-box max-w-xl">
      <form method="dialog" class="modal-corner">
        <button onclick={(event) => close(id, msg.oncancel, event)}>✕</button>
      </form>
      <!-- 消息标题 -->
      <h3 class="modal-title">
        <Icon class="size-6" />
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html pretty(msg.title || m.default_confirm_title())}
      </h3>
      <!-- 消息内容 -->
      <p class="opacity-90">
        {msg.message || m.default_confirm_message()}
      </p>
      <!-- 操作按钮 -->
      <div class="modal-action">
        <button class="btn" onclick={() => close(id, msg.oncancel)}>{m.cancel()}</button>
        <button class="btn btn-submit" onclick={() => close(id, msg.onconfirm)}>{m.confirm()}</button>
      </div>
    </div>
  </dialog>
{/each}
