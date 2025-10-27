<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { Button } from '$lib/components';
  import { m } from '$lib/paraglide/messages';
  import { deLocalizeHref } from '$lib/paraglide/runtime';
  import { entries } from '$lib/stores.svelte';
  import { formatISO8601 } from '$lib/utils';
  import { Trash } from 'phosphor-svelte';
  import { type Snippet } from 'svelte';
  import { flip } from 'svelte/animate';

  let { children }: { children: Snippet } = $props();

  // 侧边栏宽度
  const SIDEBAR_WIDTH = '12rem';
</script>

<div class="relative h-(--app-h) rounded-container p-0">
  <div class="absolute inset-y-0 left-0 flex flex-col border-r" style:width={SIDEBAR_WIDTH}>
    <div class="menu-title pt-3 text-xs tracking-wide text-base-content/60">{m.history_records()}</div>
    <div class="h-full overflow-y-auto">
      <ul class="menu w-full gap-1.5">
        {#each entries.current as entry, index (entry.id)}
          {@const href = `/histories/${entry.id}`}
          {@const active = deLocalizeHref(page.url.pathname) === href}
          <li animate:flip={{ duration: 200 }}>
            <a {href} class="group gap-1 rounded-md px-1.5 {active ? 'glass bg-emphasis text-neutral-content' : ''}">
              <kbd class="kbd kbd-sm text-primary/80">{entry.key}</kbd>
              <span class="truncate py-1 text-xs tracking-wider opacity-60">
                {entry.selection.trim() || formatISO8601(entry.datetime)}
              </span>
              <Button
                icon={Trash}
                class="hidden border-0 shadow-none group-hover:inline-flex {active ? 'hover:bg-base-300' : ''}"
                onclick={(event) => {
                  event.stopPropagation();
                  event.preventDefault();
                  // 不是当前页面，直接删除
                  if (deLocalizeHref(page.url.pathname) !== href) {
                    entries.current.splice(index, 1);
                    return;
                  }
                  // 删除后跳转到上一个日志
                  entries.current.splice(index, 1);
                  if (index > 0) {
                    goto(resolve(`/histories/${entries.current[index - 1].id}`));
                  } else {
                    goto(resolve('/histories'));
                  }
                }}
              />
            </a>
          </li>
        {/each}
      </ul>
    </div>
  </div>
  <div class="relative h-full overflow-y-auto p-4" style:margin-left={SIDEBAR_WIDTH}>
    {@render children()}
  </div>
</div>
