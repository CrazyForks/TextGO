<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import { Button, CodeMirror, confirm } from '$lib/components';
  import type { Log } from '$lib/types';
  import { markdown } from '@codemirror/lang-markdown';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { marked } from 'marked';
  import ollama from 'ollama/browser';
  import { ArrowCounterClockwise, CopySimple, Robot, TextIndent } from 'phosphor-svelte';
  import { onMount, untrack } from 'svelte';

  let { data } = $props();
  let { theme } = data;
  let darkMode = $derived(theme.current !== 'light');

  let log: Log | null = $state(null);
  let chatMode: boolean = $derived.by(() => log?.actionType === 'prompt');
  let autoScroll = $state(true);
  let mainElement: HTMLElement | null = $state(null);
  let scrollInterval: ReturnType<typeof setInterval> | undefined = $state();

  // 关闭窗口函数
  async function closeWindow() {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.hide();
    } catch (error) {
      console.error('Failed to close window:', error);
    }
  }

  // 自动滚动到底部
  function scrollToEnd() {
    if (autoScroll && mainElement) {
      mainElement.scrollTo({ top: mainElement.scrollHeight, behavior: 'smooth' });
    }
  }

  // 开始自动滚动
  function startAutoScroll() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
    scrollInterval = setInterval(scrollToEnd, 100);
  }

  // 停止自动滚动
  function stopAutoScroll() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
      scrollInterval = undefined;
    }
  }

  // 处理用户滚动事件
  function handleScroll(event: Event) {
    if (log?.streaming && log?.response) {
      const target = event.target as HTMLElement;
      // 如果用户向上滚动，停止自动滚动
      if (autoScroll) {
        const isScrollingUp = target.scrollTop < target.scrollHeight - target.clientHeight - 10;
        if (isScrollingUp) {
          autoScroll = false;
          stopAutoScroll();
        }
      } else {
        // 如果用户滚动到底部，恢复自动滚动
        const isAtBottom = target.scrollTop + target.clientHeight >= target.scrollHeight - 10;
        if (isAtBottom) {
          autoScroll = true;
          startAutoScroll();
        }
      }
    }
  }

  // 查找并绑定main元素
  onMount(() => {
    mainElement = document.querySelector('main');
    if (mainElement) {
      mainElement.addEventListener('scroll', handleScroll);
    }
    return () => {
      stopAutoScroll();
      if (mainElement) {
        mainElement.removeEventListener('scroll', handleScroll);
      }
    };
  });

  $effect(() => {
    if (chatMode && log && log.result && log.streaming === undefined) {
      untrack(async () => {
        if (!log) {
          return;
        }
        try {
          // 如果是聊天模式，请求 ollama 接口并获取回复
          log.streaming = true;
          // 开始自动滚动
          startAutoScroll();
          const messages = [{ role: 'user', content: log.result! }];
          if (log.systemPrompt && log.systemPrompt.trim().length > 0) {
            // 加入系统提示词
            messages.unshift({ role: 'system', content: log.systemPrompt });
          }
          const response = await ollama.chat({
            model: 'gemma3:4b',
            messages: messages,
            stream: true
          });
          // 保存回复内容
          log.response = '';
          for await (const part of response) {
            if (log?.leaving) {
              // 已经确认离开，停止接收
              break;
            }
            log.response += part.message.content;
          }
        } catch (error) {
          console.error(error);
          if (error instanceof Error) {
            log.response = error.message;
          }
        } finally {
          log.streaming = false;
          // 停止自动滚动
          stopAutoScroll();
        }
      });
    }
  });

  // 路由跳转前，如果有对话正在进行中，弹出确认对话框
  beforeNavigate(({ to, cancel }) => {
    if (log?.leaving) {
      // 已经确认离开，直接放行
      return;
    }
    if (log?.streaming) {
      cancel();
      confirm({
        message: 'AI 正在生成回复，确定要离开吗？',
        onconfirm: () => {
          ollama.abort();
          if (log) {
            log.leaving = true;
            log.streaming = false;
          }
          to && goto(to.url);
        }
      });
    }
  });

  onMount(() => {
    // 监听主进程发送的事件
    const unlisten = listen<string>('log', (event) => {
      log = JSON.parse(event.payload) as Log;
    });
    return () => {
      log = null;
      unlisten.then((fn) => fn());
    };
  });
</script>

{#key log?.id}
  <main class="h-screen w-screen overflow-hidden">
    <div class="flex h-8 items-center justify-between gap-2 bg-base-300 pr-2 pl-20" data-tauri-drag-region>
      <div class="pointer-events-none flex items-center gap-2 truncate">
        {#if chatMode}
          <Robot class="size-4.5 shrink-0" />
          <span class="truncate text-sm text-base-content/80">gemma3:4b</span>
        {/if}
      </div>
      <div class="flex items-center gap-1">
        {#if !chatMode}
          <Button icon={ArrowCounterClockwise} onclick={closeWindow} />
          <Button icon={TextIndent} onclick={closeWindow} />
          <Button icon={CopySimple} onclick={closeWindow} />
        {/if}
      </div>
    </div>
    <div class="size-full overflow-auto">
      {#if chatMode}
        <div class="p-2">
          {#if log?.streaming}
            <div class="loading mb-2 loading-sm loading-dots opacity-70"></div>
          {/if}
          {#if log?.response}
            <div class="prose prose-sm max-w-none text-base-content/90">
              {@html marked(log.response + (log?.streaming ? ' |' : ''))}
            </div>
          {:else if !log?.streaming}
            <div class="text-sm text-base-content/60 italic">等待回复...</div>
          {/if}
        </div>
      {:else}
        <CodeMirror
          minHeight="calc(100vh - 2.25rem)"
          maxHeight="calc(100vh - 2.25rem)"
          class="rounded-none border-none"
          panelClass="hidden"
          language={chatMode ? markdown() : undefined}
          document={log?.result}
          {darkMode}
        />
      {/if}
    </div>
  </main>
{/key}
