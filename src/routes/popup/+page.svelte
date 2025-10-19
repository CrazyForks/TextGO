<script lang="ts">
  import { Button, CodeMirror } from '$lib/components';
  import { ollamaHost } from '$lib/stores.svelte';
  import type { Entry } from '$lib/types';
  import { listen } from '@tauri-apps/api/event';
  import { marked } from 'marked';
  import { Ollama } from 'ollama';
  import { ArrowCounterClockwise, CopySimple, Robot, TextIndent } from 'phosphor-svelte';
  import { onMount, untrack } from 'svelte';

  // 快捷键触发记录
  let entry: Entry | null = $state(null);

  // CodeMirror 实例
  let codeMirror: CodeMirror | null = $state(null);

  // 是否为提示词模式
  let promptMode: boolean = $derived.by(() => entry?.actionType === 'prompt');

  // 流式传输状态
  let streaming: boolean | null = $state(null);

  // Ollama 实例
  let ollama = new Ollama();
  $effect(() => {
    ollama = new Ollama({ host: ollamaHost.current || undefined });
  });

  // 自动滚动状态
  let autoScroll = $state(false);

  // 滚动容器元素
  let scrollElement: HTMLElement | null = $state(null);

  // 滚动定时器
  let scrollInterval: ReturnType<typeof setInterval> | null = $state(null);

  // 开启对话
  $effect(() => {
    if (promptMode && streaming === null) {
      untrack(async () => {
        if (!entry || !entry.result || !entry.model) {
          return;
        }
        try {
          // 开始流式传输
          streaming = true;
          // 开始自动滚动
          startAutoScroll();
          // 添加用户提示词
          const messages = [{ role: 'user', content: entry.result }];
          // 添加系统提示词
          const systemPrompt = entry.systemPrompt?.trim();
          if (systemPrompt) {
            messages.unshift({ role: 'system', content: systemPrompt });
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
              // 中止流式传输
              break;
            }
            entry.response += part.message.content;
          }
        } catch (error) {
          if (error instanceof Error && error.name !== 'AbortError') {
            entry.response = error.message || '发生未知错误';
          }
        } finally {
          // 结束流式传输
          streaming = false;
          // 停止自动滚动
          stopAutoScroll();
        }
      });
    }
  });

  /**
   * 开始自动滚动
   */
  function startAutoScroll() {
    if (scrollInterval) {
      clearInterval(scrollInterval);
    }
    autoScroll = true;
    scrollInterval = setInterval(() => {
      if (autoScroll && scrollElement) {
        scrollElement.scrollTo({
          top: scrollElement.scrollHeight,
          behavior: 'smooth'
        });
      }
    }, 100);
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

  onMount(() => {
    const setup = (data: Entry | null) => {
      entry = data;
      streaming && ollama.abort();
      streaming = null;
      autoScroll && stopAutoScroll();
      autoScroll = false;
    };
    // 监听主进程发送的事件
    const unlisten = listen<string>('popup', (event) => {
      setup(JSON.parse(event.payload) as Entry);
    });
    return () => {
      setup(null);
      unlisten.then((fn) => fn());
    };
  });
</script>

{#key entry?.id}
  <main class="h-screen w-screen overflow-hidden">
    <div class="flex h-8 items-center justify-between gap-2 bg-base-300 pr-2 pl-20" data-tauri-drag-region>
      <div class="pointer-events-none flex items-center gap-2 truncate">
        {#if promptMode}
          <Robot class="size-4.5 shrink-0" />
          <span class="truncate text-sm text-base-content/80">{entry?.model}</span>
        {/if}
      </div>
      <div class="flex items-center gap-1">
        {#if !promptMode}
          <Button icon={ArrowCounterClockwise} onclick={() => codeMirror?.reset()} />
          <Button icon={TextIndent} onclick={() => codeMirror?.format()} />
          <Button icon={CopySimple} onclick={() => codeMirror?.copy()} />
        {/if}
      </div>
    </div>
    <div class="size-full overflow-auto" bind:this={scrollElement} onscroll={handleScroll}>
      {#if promptMode}
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
          document={entry?.result}
          bind:this={codeMirror}
        />
      {/if}
    </div>
  </main>
{/key}
