<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { Button, confirm } from '$lib/components';
  import { formatISO8601 } from '$lib/utils';
  import ollama from 'ollama/browser';
  import { GearSix, Trash } from 'phosphor-svelte';
  import { type Snippet } from 'svelte';
  import { flip } from 'svelte/animate';
  import type { LayoutData } from './$types';

  let { data, children }: { data: LayoutData; children: Snippet } = $props();

  /**
   * 侧边栏宽度
   */
  const SIDEBAR_WIDTH = '12rem';

  /**
   * 侧边栏激活样式
   *
   * @param pathname - 路径名
   */
  const activeMenu = (pathname: string) => {
    return page.url.pathname === pathname
      ? 'bg-base-100 gradient shadow-sm text-base-content/90'
      : 'border-none text-base-content/60 hover:text-base-content transition-colors';
  };
</script>

<div class="h-[calc(100vh-3.35rem)] rounded-box bg-base-100">
  <div class="fixed inset-y-0 left-0 border-r" style:width={SIDEBAR_WIDTH}>
    <div class="h-8" data-tauri-drag-region></div>
    <div class="flex h-[calc(100%-2rem)] flex-col">
      <div class="menu-title text-xs tracking-wide text-base-content/60">历史记录</div>
      <div class="overflow-y-auto">
        <ul class="menu w-full gap-1.5">
          {#each data.logs.current as log, index (log.id)}
            {@const href = `/histories/${log.id}`}
            <li animate:flip={{ duration: 200 }}>
              <a {href} class="group gap-1 rounded-md px-1.5 {activeMenu(href)}">
                <kbd class="kbd kbd-sm text-primary/80">{log.key}</kbd>
                <span class="truncate py-1 text-xs tracking-wider opacity-60">
                  {log.selection.trim() || formatISO8601(log.datetime)}
                </span>
                <Button
                  icon={Trash}
                  class="hidden border-0 shadow-none group-hover:inline-flex"
                  onclick={(event) => {
                    event.stopPropagation();
                    event.preventDefault();
                    if (page.url.pathname !== href) {
                      data.logs.current.splice(index, 1);
                      return;
                    }
                    const gotoPrev = () => {
                      // 删除后跳转到上一个日志，若无则跳转到设置页
                      const prev = index > 0 ? `/histories/${data.logs.current[index - 1].id}` : '/histories';
                      data.logs.current.splice(index, 1);
                      goto(prev);
                    };
                    if (log.streaming) {
                      confirm({
                        message: 'AI 正在生成回复，确定要离开吗？',
                        onconfirm: () => {
                          ollama.abort();
                          log.leaving = true;
                          log.streaming = false;
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
  </div>
  <div class="ml px-2 pt-0.5 pb-2" style:margin-left={SIDEBAR_WIDTH}>
    {@render children()}
  </div>
</div>
