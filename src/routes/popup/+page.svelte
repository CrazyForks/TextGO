<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import { CodeMirror, confirm } from '$lib/components';
  import type { Log } from '$lib/types';
  import { markdown } from '@codemirror/lang-markdown';
  import { listen } from '@tauri-apps/api/event';
  import { Window } from '@tauri-apps/api/window';
  import { marked } from 'marked';
  import ollama from 'ollama/browser';
  import { ChatText, FileJs, FilePy, Robot, X } from 'phosphor-svelte';
  import { onMount, untrack } from 'svelte';
  import { fade } from 'svelte/transition';

  let log: Log | null = $state(null);
  let chatMode: boolean = $derived.by(() => log?.actionType === 'prompt');
  let autoScroll = $state(true);
  let mainElement: HTMLElement | null = $state(null);
  let scrollInterval: ReturnType<typeof setInterval> | undefined = $state();
  let clipboardExpanded = $state(false);
  let selectionExpanded = $state(false);

  // 关闭窗口函数
  async function closeWindow() {
    try {
      const appWindow = Window.getCurrent();
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
    // 监听主进程发送的 show-log 事件
    const unlisten = listen<Log>('log', (event) => {
      log = event.payload;
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

{#key log?.id}
  <div class="relative size-full rounded-lg border border-white/30 bg-white/20 shadow-xl backdrop-blur-md">
    <!-- 关闭按钮 -->
    <button
      onclick={closeWindow}
      class="group absolute top-3 right-3 z-50 rounded-full bg-black/20 p-2 transition-colors duration-200 hover:bg-black/30"
      title="关闭窗口"
    >
      <X size={16} class="text-white/80 group-hover:text-white" />
    </button>

    <!-- 内容区域 -->
    <div class="h-full overflow-auto p-4 pt-12" data-tauri-drag-region>
      {#if chatMode}
        {#if log?.streaming}
          <div class="loading mb-2 loading-sm loading-dots opacity-70"></div>
        {/if}
        {#if log?.response}
          <div class="prose prose-sm max-w-none text-white/90">
            {@html marked(log.response + (log?.streaming ? ' |' : ''))}
          </div>
        {:else if !log?.streaming}
          <div class="text-sm text-white/60 italic">等待回复...</div>
        {/if}
      {:else}
        <div class="h-full">
          <CodeMirror
            minHeight="calc(100vh - 3rem)"
            maxHeight="calc(100vh - 3rem)"
            language={chatMode ? markdown() : undefined}
            darkMode={true}
            document={log?.result}
          />
        </div>
      {/if}
    </div>
  </div>
{/key}

<style>
  :global {
    html {
      background: rgba(255, 255, 255, 0.1); /* 半透明白色背景 */
      backdrop-filter: blur(10px); /* 磨砂效果 */
      -webkit-backdrop-filter: blur(10px); /* Safari 兼容 */
      border-radius: 12px; /* 圆角 */
      border: 1px solid rgba(255, 255, 255, 0.2); /* 边框 */
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1); /* 阴影 */
    }
  }
</style>
