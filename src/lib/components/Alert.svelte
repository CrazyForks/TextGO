<script lang="ts" module>
  import { CheckCircle, Info, Warning, XCircle, type IconComponentProps } from 'phosphor-svelte';
  import type { Component } from 'svelte';

  export type Message = {
    /** 提示消息级别 */
    level?: 'info' | 'success' | 'warning' | 'error';
    /** 提示消息内容 */
    message: string;
    /** 提示显示时间（毫秒） */
    timeout?: number;
    /** 提示消息是否唯一 */
    unique?: boolean;
  };

  export type AlertProps = {
    /** 是否在对话框中显示 */
    dialog?: boolean;
    /** 显示提示的最大数量 */
    maxSize?: number;
    /** 提示的默认显示时间（毫秒） */
    timeout?: number;
  };

  /**
   * 当前对话框标识
   */
  let _dialogId: string = $state('');

  /**
   * 当前消息的响应式存储
   */
  let message: Message | null = $state(null);

  /**
   * 显示提示消息
   *
   * @param msg - 提示消息实例
   */
  export function alert(msg: Message | string | null) {
    if (typeof msg === 'string') {
      msg = { message: msg };
    }
    message = msg;
  }

  /**
   * 提示级别到颜色和图标的映射
   */
  const mappings: Record<string, { color: string; icon: Component<IconComponentProps> }> = {
    info: { color: 'var(--color-info)', icon: Info },
    success: { color: 'var(--color-success)', icon: CheckCircle },
    warning: { color: 'var(--color-warning)', icon: Warning },
    error: { color: 'var(--color-error)', icon: XCircle }
  };
</script>

<script lang="ts">
  import { onMount } from 'svelte';
  import { SvelteMap } from 'svelte/reactivity';
  import { fade, fly, slide } from 'svelte/transition';

  const { dialog = false, maxSize = 1, timeout = 2000 }: AlertProps = $props();
  const dialogId = dialog ? crypto.randomUUID() : '';

  // 提示消息的响应式映射
  const alerts = new SvelteMap<string, Message>();

  // 判断是否在当前组件中显示
  let instance = $derived(dialogId === _dialogId ? message : null);
  $effect(() => {
    if (instance) {
      const id = instance.unique ? instance.message : crypto.randomUUID();
      alerts.set(id, instance);
      setTimeout(() => alerts.delete(id), instance.timeout || timeout);
      alert(null);
    }
    return () => {
      if (alerts.size > maxSize) {
        const [id] = alerts.keys();
        alerts.delete(id);
      }
    };
  });

  // 更新当前对话框标识
  onMount(() => {
    const temp = _dialogId;
    _dialogId = dialogId;
    return () => (_dialogId = temp);
  });
</script>

{#if alerts.size > 0}
  <div class="toast-top toast-center pointer-events-none toast top-14 z-100" in:fly={{ y: -50 }}>
    {#each Array.from(alerts.entries()).reverse() as [id, alert] (id)}
      {@const { icon: Icon, color } = mappings[alert.level ?? 'success']}
      <div
        role="alert"
        class="alert animate-none gap-2 border bg-base-150/95 px-3 py-2 shadow-lg"
        in:slide={{ duration: 100 }}
        out:fade
      >
        <Icon {color} class="size-6" />
        <span>{alert.message}</span>
      </div>
    {/each}
  </div>
{/if}
