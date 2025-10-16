<script lang="ts">
  import { Button, CodeMirror } from '$lib/components';
  import type { Entry } from '$lib/types';
  import { markdown } from '@codemirror/lang-markdown';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { marked } from 'marked';
  import ollama from 'ollama/browser';
  import { ArrowCounterClockwise, CopySimple, Robot, TextIndent } from 'phosphor-svelte';
  import { onMount, untrack } from 'svelte';

  let entry: Entry | null = $state(null);
  let chatMode: boolean = $derived.by(() => entry?.actionType === 'prompt');

  let autoScroll = $state(true);
  let scrollElement: HTMLElement | null = $state(null);
  let scrollInterval: ReturnType<typeof setInterval> | null = $state(null);

  /**
   * 关闭当前窗口
   */
  async function closeWindow() {
    try {
      const currentWindow = getCurrentWindow();
      await currentWindow.hide();
    } catch (error) {
      console.error('关闭当前窗口失败:', error);
    }
  }

  /**
   * 自动滚动到底部
   */
  function scrollToEnd() {
    if (autoScroll && scrollElement) {
      scrollElement.scrollTo({
        top: scrollElement.scrollHeight,
        behavior: 'smooth'
      });
    }
  }

  /**
   * 开始自动滚动
   */
  function startAutoScroll() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
    autoScroll = true;
    scrollInterval = setInterval(scrollToEnd, 100);
  }

  /**
   * 停止自动滚动
   */
  function stopAutoScroll() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
    autoScroll = false;
    scrollInterval = null;
  }

  /**
   * 处理用户滚动事件
   *
   * @param event - 滚动事件
   */
  function handleScroll(event: Event) {
    if (streaming && entry?.response) {
      const target = event.target as HTMLElement;
      if (autoScroll) {
        // 如果用户向上滚动，停止自动滚动
        const isScrollingUp = target.scrollTop + target.clientHeight < target.scrollHeight - 10;
        if (isScrollingUp) {
          stopAutoScroll();
        }
      } else {
        // 如果用户滚动到底部，恢复自动滚动
        const isAtBottom = target.scrollTop + target.clientHeight >= target.scrollHeight - 10;
        if (isAtBottom) {
          startAutoScroll();
        }
      }
    }
  }

  let streaming: boolean | null = $state(null);

  $effect(() => {
    if (chatMode && entry && entry.result && streaming === null) {
      untrack(async () => {
        if (!entry || !entry.model) {
          return;
        }
        try {
          // 如果是聊天模式，请求 ollama 接口并获取回复
          streaming = true;
          // 开始自动滚动
          startAutoScroll();
          const messages = [{ role: 'user', content: entry.result! }];
          if (entry.systemPrompt && entry.systemPrompt.trim().length > 0) {
            // 加入系统提示词
            messages.unshift({ role: 'system', content: entry.systemPrompt });
          }
          const response = await ollama.chat({
            model: entry.model,
            messages: messages,
            stream: true
          });
          // 保存回复内容
          entry.response = '';
          for await (const part of response) {
            if (streaming === null) {
              // 已经确认离开，停止接收
              break;
            }
            entry.response += part.message.content;
          }
        } catch (error) {
          console.error(error);
          if (error instanceof Error) {
            entry.response = error.message;
          }
        } finally {
          streaming = false;
          // 停止自动滚动
          stopAutoScroll();
        }
      });
    }
  });

  onMount(() => {
    // 监听主进程发送的事件
    const unlisten = listen<string>('popup', (event) => {
      entry = JSON.parse(event.payload) as Entry;
      if (streaming) {
        ollama.abort();
      }
      streaming = null;
    });
    return () => {
      if (streaming) {
        ollama.abort();
      }
      entry = null;
      stopAutoScroll();
      unlisten.then((fn) => fn());
    };
  });
</script>

{#key entry?.id}
  <main class="h-screen w-screen overflow-hidden">
    <div class="flex h-8 items-center justify-between gap-2 bg-base-300 pr-2 pl-20" data-tauri-drag-region>
      <div class="pointer-events-none flex items-center gap-2 truncate">
        {#if chatMode}
          <Robot class="size-4.5 shrink-0" />
          <span class="truncate text-sm text-base-content/80">{entry?.model}</span>
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
    <div class="size-full overflow-auto" bind:this={scrollElement} onscroll={handleScroll}>
      {#if chatMode}
        <div class="px-4 pt-2 pb-10">
          {#if streaming && !entry?.response}
            <div class="loading loading-sm loading-dots opacity-70"></div>
          {:else if entry?.response}
            <div class="prose prose-sm max-w-none text-base-content/90">
              {@html marked(entry.response + (streaming ? ' |' : ''))}
            </div>
          {/if}
        </div>
      {:else}
        <CodeMirror
          minHeight="calc(100vh - 2.25rem)"
          maxHeight="calc(100vh - 2.25rem)"
          class="rounded-none border-none"
          panelClass="hidden"
          language={chatMode ? markdown() : undefined}
          document={entry?.result}
        />
      {/if}
    </div>
  </main>
{/key}
