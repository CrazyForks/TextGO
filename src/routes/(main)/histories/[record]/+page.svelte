<script lang="ts">
  import { page } from '$app/state';
  import { CodeMirror } from '$lib/components';
  import { entries } from '$lib/states.svelte';
  import type { Entry } from '$lib/types';
  import { formatISO8601 } from '$lib/utils';
  import { markdown } from '@codemirror/lang-markdown';
  import { marked } from 'marked';
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
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';

  let entry: Entry | undefined = $derived(entries.current.find((e) => e.id === page.params.record));
  let chatMode: boolean = $derived.by(() => entry?.actionType === 'prompt');
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
    if (entry?.streaming && entry?.response) {
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
</script>

{#key entry?.id}
  <ul class="timeline timeline-vertical timeline-compact timeline-snap-icon" in:fade>
    <li>
      <div class="timeline-middle">
        <Lightning class="size-5 opacity-60" />
      </div>
      <div class="timeline-end mb-8 pt-[1px]">
        <time class="pr-1 text-sm italic opacity-50">{formatISO8601(entry?.datetime)}</time>
      </div>
      <hr />
    </li>
    {#if entry?.clipboard}
      <li>
        <hr />
        <div class="timeline-middle">
          <ClipboardText class="size-5" />
        </div>
        <div class="timeline-end mb-8 w-[calc(100%-2rem)] pt-1">
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
              class="m-2 max-h-48 overflow-auto text-xs whitespace-pre opacity-80"
              transition:fade={{ duration: 200 }}
            >
              {entry.clipboard}
            </div>
          {/if}
        </div>
        <hr />
      </li>
    {/if}
    {#if entry?.selection}
      <li>
        <hr />
        <div class="timeline-middle">
          <Textbox class="size-5" />
        </div>
        <div class="timeline-end mb-8 w-[calc(100%-2rem)] pt-1">
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
            {#if entry?.caseLabel}
              <span class="ml-2 badge gap-0.5 border badge-xs">
                <FingerprintSimple class="size-3" />
                {entry?.caseLabel}
              </span>
            {/if}
          </div>
          {#if selectionExpanded}
            <div
              class="m-2 max-h-96 overflow-auto text-xs whitespace-pre opacity-80"
              transition:fade={{ duration: 200 }}
            >
              {entry.selection}
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
        {:else if entry?.scriptLang === 'javascript'}
          <FileJs class="size-5" />
        {:else if entry?.scriptLang === 'python'}
          <FilePy class="size-5" />
        {/if}
      </div>
      <div class="timeline-end w-[calc(100%-2rem)] pt-[1px] {chatMode ? 'mb-2' : 'mb-8'}">
        <time class="text-sm font-semibold italic">{entry?.actionLabel}</time>
        <div class="mt-2">
          <CodeMirror language={chatMode ? markdown() : undefined} darkMode={!chatMode} document={entry?.result} />
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
        <div class="timeline-end mb-8 w-[calc(100%-2rem)] pt-[1px]">
          <time class="text-sm font-semibold italic">AI 回复</time>
          <div class="mt-2 rounded-box border bg-base-150 p-4">
            {#if entry?.streaming}
              <div class="loading mb-2 loading-sm loading-dots opacity-70"></div>
            {/if}
            {#if entry?.response}
              <div class="prose prose-sm max-w-none text-base-content">
                {@html marked(entry.response + (entry?.streaming ? ' |' : ''))}
              </div>
            {:else if !entry?.streaming}
              <div class="text-sm text-base-content/50 italic">等待回复...</div>
            {/if}
          </div>
        </div>
      </li>
    {/if}
  </ul>
{/key}
