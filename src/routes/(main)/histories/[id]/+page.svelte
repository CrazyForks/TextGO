<script lang="ts">
  import { page } from '$app/state';
  import { CodeMirror } from '$lib/components';
  import { m } from '$lib/paraglide/messages';
  import { entries } from '$lib/stores.svelte';
  import type { Entry } from '$lib/types';
  import { formatISO8601 } from '$lib/utils';
  import { markdown } from '@codemirror/lang-markdown';
  import {
    CaretDown,
    CaretRight,
    ClipboardText,
    Clock,
    Cube,
    Empty,
    FileJs,
    FileMd,
    FilePy,
    FingerprintSimple,
    Textbox
  } from 'phosphor-svelte';
  import { fade } from 'svelte/transition';

  let entry: Entry | undefined = $derived(entries.current.find((e) => e.id === page.params.id));
  let promptMode: boolean = $derived.by(() => entry?.actionType === 'prompt');
  let clipboardExpanded = $state(false);
  let selectionExpanded = $state(false);
</script>

{#key entry?.id}
  <ul class="timeline timeline-vertical timeline-compact timeline-snap-icon" in:fade>
    <li>
      <div class="timeline-middle">
        <Clock class="size-5 opacity-50" />
      </div>
      <div class="timeline-end mb-10 w-[calc(100%-0.5rem)] pt-[1px]">
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
        <div class="timeline-end mb-10 w-[calc(100%-0.5rem)] pt-1">
          <div class="flex items-center gap-2">
            <button class="flex cursor-pointer items-center" onclick={() => (clipboardExpanded = !clipboardExpanded)}>
              <span class="pr-1 text-sm font-semibold italic">{m.clipboard_text()}</span>
              <span class="inline-flex">
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
              class="m-2 max-h-96 overflow-auto text-xs whitespace-pre opacity-80"
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
        <div class="timeline-end mb-10 w-[calc(100%-0.5rem)] pt-1">
          <div class="flex items-center gap-2">
            <button class="flex cursor-pointer items-center" onclick={() => (selectionExpanded = !selectionExpanded)}>
              <span class="pr-1 text-sm font-semibold italic">{m.selected_text()}</span>
              <span class="inline-flex">
                {#if selectionExpanded}
                  <CaretDown class="size-4" />
                {:else}
                  <CaretRight class="size-4" />
                {/if}
              </span>
            </button>
            {#if entry?.caseLabel}
              <span class="badge shrink-0 gap-0.5 border badge-xs">
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
        {#if promptMode}
          <FileMd class="size-5" />
        {:else if entry?.scriptLang === 'javascript'}
          <FileJs class="size-5" />
        {:else if entry?.scriptLang === 'python'}
          <FilePy class="size-5" />
        {/if}
      </div>
      <div class="timeline-end w-[calc(100%-0.5rem)] pt-1">
        <div class="flex items-center gap-2">
          <time class="pr-1 text-sm font-semibold italic">{entry?.actionLabel}</time>
          {#if promptMode}
            <span class="badge shrink-0 gap-0.5 border badge-xs">
              <Cube class="size-3" />
              {entry?.model}
            </span>
          {:else if entry?.quietMode}
            <span class="badge shrink-0 gap-0.5 border badge-xs">
              <Empty class="size-3" />
              {m.quiet_mode()}
            </span>
          {/if}
        </div>
        <CodeMirror
          title={promptMode ? m.prompt() : m.script_result()}
          language={promptMode ? markdown() : undefined}
          document={entry?.result}
          class="mt-2"
        />
      </div>
    </li>
  </ul>
{/key}
