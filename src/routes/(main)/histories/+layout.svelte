<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { Button, confirm } from '$lib/components';
  import { formatISO8601 } from '$lib/utils';
  import ollama from 'ollama/browser';
  import { Trash } from 'phosphor-svelte';
  import { type Snippet } from 'svelte';
  import { flip } from 'svelte/animate';
  import type { LayoutData } from './$types';

  let { data, children }: { data: LayoutData; children: Snippet } = $props();

  /**
   * 侧边栏宽度
   */
  const SIDEBAR_WIDTH = '12rem';
</script>

<div class="relative h-(--app-h) rounded-container">
  <div class="absolute inset-y-0 left-0 flex flex-col border-r" style:width={SIDEBAR_WIDTH}>
    <div class="menu-title text-xs tracking-wide text-base-content/60">历史记录</div>
    <div class="h-full overflow-y-auto">
      <ul class="menu w-full gap-1.5">
        {#each data.entries.current as entry, index (entry.id)}
          {@const href = `/histories/${entry.id}`}
          {@const menuActive = page.url.pathname === href}
          <li animate:flip={{ duration: 200 }}>
            <a
              {href}
              class="group gap-1 rounded-md px-1.5 {menuActive ? 'glass bg-emphasis text-neutral-content' : ''}"
            >
              <kbd class="kbd kbd-sm text-primary/80">{entry.key}</kbd>
              <span class="truncate py-1 text-xs tracking-wider opacity-60">
                {entry.selection.trim() || formatISO8601(entry.datetime)}
              </span>
              <Button
                icon={Trash}
                class="hidden border-0 shadow-none group-hover:inline-flex {menuActive ? 'hover:bg-base-300' : ''}"
                onclick={(event) => {
                  event.stopPropagation();
                  event.preventDefault();
                  if (page.url.pathname !== href) {
                    data.entries.current.splice(index, 1);
                    return;
                  }
                  const gotoPrev = () => {
                    // 删除后跳转到上一个日志，若无则跳转到设置页
                    const prev = index > 0 ? `/histories/${data.entries.current[index - 1].id}` : '/histories';
                    data.entries.current.splice(index, 1);
                    goto(prev);
                  };
                  if (entry.streaming) {
                    confirm({
                      message: 'AI 正在生成回复，确定要离开吗？',
                      onconfirm: () => {
                        ollama.abort();
                        entry.leaving = true;
                        entry.streaming = false;
                        gotoPrev();
                      }
                    });
                  } else {
                    gotoPrev();
                  }
                }}
              />
            </a>
          </li>
        {/each}
      </ul>
    </div>
  </div>
  <div class="relative h-full overflow-y-auto" style:margin-left={SIDEBAR_WIDTH}>
    {@render children()}
  </div>
</div>
