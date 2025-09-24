<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import { page } from '$app/state';
  import { CodeMirror, confirm } from '$lib/components';
  import { logs } from '$lib/states.svelte';
  import type { Log } from '$lib/types';
  import { formatISO8601 } from '$lib/utils';
  import { markdown } from '@codemirror/lang-markdown';
  import { marked } from 'marked';
  import ollama from 'ollama/browser';
  import {
    CaretDown,
    CaretRight,
    ChatText,
    ClipboardText,
    FileJs,
    FilePy,
    FingerprintSimple,
    Lightning,
    Robot,
    Textbox
  } from 'phosphor-svelte';
  import { onMount, untrack } from 'svelte';
  import { fade } from 'svelte/transition';

  let log: Log | undefined = $derived(logs.current.find((l) => l.id === page.params.record));
  let chatMode: boolean = $derived.by(() => log?.actionType === 'prompt');
  let autoScroll = $state(true);
  let mainElement: HTMLElement | null = $state(null);
  let scrollInterval: ReturnType<typeof setInterval> | undefined = $state();
  let clipboardExpanded = $state(false);
  let selectionExpanded = $state(false);

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
          log.leaving = true;
          log.streaming = false;
          to && goto(to.url);
        }
      });
    }
  });
</script>

{#key log?.id}
  <ul class="timeline timeline-vertical timeline-compact h-[calc(100dvh-2rem)] p-2 timeline-snap-icon" in:fade>
    <li>
      <div class="timeline-middle">
        <Lightning class="size-5 opacity-60" />
      </div>
      <div class="timeline-end mb-8 pt-[1px]">
        <time class="pr-1 text-sm italic opacity-50">{formatISO8601(log?.datetime)}</time>
      </div>
      <hr />
    </li>
    {#if log?.clipboard}
      <li>
        <hr />
        <div class="timeline-middle">
          <ClipboardText class="size-5" />
        </div>
        <div class="timeline-end mb-8 pt-1">
          <div class="flex items-center">
            <button class="flex cursor-pointer items-center" onclick={() => (clipboardExpanded = !clipboardExpanded)}>
              <span class="text-sm font-semibold italic">剪贴版</span>
              <span class="ml-1 inline-flex">
                {#if clipboardExpanded}
                  <CaretDown class="size-4" />
                {:else}
                  <CaretRight class="size-4" />
                {/if}
              </span>
            </button>
          </div>
          {#if clipboardExpanded}
            <div
              class="m-2 max-h-48 w-[calc(100vw-15rem)] overflow-auto text-xs whitespace-pre opacity-80"
              transition:fade={{ duration: 200 }}
            >
              {log.clipboard}
            </div>
          {/if}
        </div>
        <hr />
      </li>
    {/if}
    {#if log?.selection}
      <li>
        <hr />
        <div class="timeline-middle">
          <Textbox class="size-5" />
        </div>
        <div class="timeline-end mb-8 pt-1">
          <div class="flex items-center">
            <button class="flex cursor-pointer items-center" onclick={() => (selectionExpanded = !selectionExpanded)}>
              <span class="text-sm font-semibold italic">选中文本</span>
              <span class="ml-1 inline-flex">
                {#if selectionExpanded}
                  <CaretDown class="size-4" />
                {:else}
                  <CaretRight class="size-4" />
                {/if}
              </span>
            </button>
            {#if log?.caseLabel}
              <span class="ml-2 badge gap-0.5 border badge-xs">
                <FingerprintSimple class="size-3" />
                {log?.caseLabel}
              </span>
            {/if}
          </div>
          {#if selectionExpanded}
            <div
              class="m-2 max-h-96 w-[calc(100vw-15rem)] overflow-auto text-xs whitespace-pre opacity-80"
              transition:fade={{ duration: 200 }}
            >
              {log.selection}
            </div>
          {/if}
        </div>
        <hr />
      </li>
    {/if}
    <li>
      <hr />
      <div class="timeline-middle">
        {#if chatMode}
          <ChatText class="size-5" />
        {:else if log?.scriptLang === 'javascript'}
          <FileJs class="size-5" />
        {:else if log?.scriptLang === 'python'}
          <FilePy class="size-5" />
        {/if}
      </div>
      <div class="timeline-end pt-[1px] {chatMode ? 'mb-2' : 'mb-8'}">
        <time class="text-sm font-semibold italic">{log?.actionLabel}</time>
        <div class="mt-2 w-[calc(100vw-15rem)]">
          <CodeMirror language={chatMode ? markdown() : undefined} darkMode={!chatMode} document={log?.result} />
        </div>
      </div>
      {#if chatMode}
        <hr />
      {/if}
    </li>
    {#if chatMode}
      <li>
        <hr />
        <div class="timeline-middle">
          <Robot class="size-5" />
        </div>
        <div class="timeline-end mb-8 pt-[1px]">
          <time class="text-sm font-semibold italic">AI 回复</time>
          <div class="mt-2 w-[calc(100vw-15rem)] rounded-box border bg-base-150 p-4">
            {#if log?.streaming}
              <div class="loading mb-2 loading-sm loading-dots opacity-70"></div>
            {/if}
            {#if log?.response}
              <div class="prose prose-sm max-w-none text-base-content">
                {@html marked(log.response + (log?.streaming ? ' |' : ''))}
              </div>
            {:else if !log?.streaming}
              <div class="text-sm text-base-content/50 italic">等待回复...</div>
            {/if}
          </div>
        </div>
      </li>
    {/if}
  </ul>
{/key}
