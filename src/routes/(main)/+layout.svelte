<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { Button, Title } from '$lib/components';
  import { GitHub, Moon, Sun } from '$lib/icons';
  import { theme } from '$lib/stores.svelte';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { ClockCounterClockwise, GearSix, Keyboard } from 'phosphor-svelte';
  import { onMount, type Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();

  /**
   * 导航栏菜单项
   */
  const menus = [
    { text: '历史', icon: ClockCounterClockwise, path: '/histories' },
    { text: '快捷键', icon: Keyboard, path: '/shortcuts' },
    { text: '设置', icon: GearSix, path: '/settings' }
  ];

  /**
   * 标题栏底部边框显示状态
   */
  let titleBorder = $state(false);
  let sentinelElement: HTMLElement;

  onMount(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        // 当 sentinel 元素不可见时，显示标题栏底部边框
        titleBorder = !entry.isIntersecting;
      },
      {
        // 44px 是 Title 组件的高度
        rootMargin: '-44px 0px 0px 0px'
      }
    );
    observer.observe(sentinelElement);
    return () => {
      observer.disconnect();
    };
  });

  onMount(() => {
    const oninvalid = (event: Event) => {
      if (event.target instanceof HTMLInputElement) {
        // 阻止默认验证气泡显示
        event.preventDefault();
        // 聚焦到验证失败的输入框
        if (document.activeElement !== event.target) {
          event.target.focus();
        }
      }
    };
    // 监听所有验证失败事件
    document.addEventListener('invalid', oninvalid, true);
    return () => {
      // 清理事件监听器
      document.removeEventListener('invalid', oninvalid, true);
    };
  });

  onMount(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        // 某些情况下，隐藏窗口显示后页面内会有元素意外获得焦点，这里将其取消掉
        const activeElement = document.activeElement as HTMLElement;
        if (activeElement && activeElement !== document.body) {
          activeElement.blur();
        }
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });

  onMount(() => {
    // 监听跳转快捷键注册页面事件
    const unlisten = listen('goto-shortcuts', async () => {
      goto('/shortcuts');
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

<main class="h-screen w-screen overflow-auto overscroll-none bg-base-300">
  <Title class="sticky top-0 z-101 bg-base-300/80 backdrop-blur-sm {titleBorder ? 'border-b' : ''}">
    {#snippet fallback()}
      <!-- 导航菜单 -->
      <span class="mx-auto flex gap-2 pl-16">
        {#each menus as menu (menu.path)}
          {@const active = page.url.pathname.startsWith(menu.path)}
          <Button
            size="sm"
            square={false}
            icon={menu.icon}
            text={menu.text}
            weight={active ? 'fill' : 'light'}
            class="border-none transition-all hover:bg-base-100 {active
              ? 'gradient bg-base-100 shadow-around'
              : 'text-base-content/80 hover:text-base-content'}"
            onclick={() => goto(menu.path)}
          ></Button>
        {/each}
      </span>
      <!-- 主题切换 -->
      <label class="swap mr-2 swap-rotate opacity-50 transition-opacity hover:opacity-100">
        <input
          type="checkbox"
          class="theme-controller"
          checked={theme.current === 'light'}
          onclick={(event) => {
            const isLight = (event.target as HTMLInputElement).checked;
            theme.current = isLight ? 'light' : 'dracula';
          }}
        />
        <Moon class="swap-off size-5" />
        <Sun class="swap-on size-5" />
      </label>
      <!-- GitHub 链接 -->
      <button
        class="mr-4 cursor-pointer opacity-50 transition-opacity hover:opacity-100"
        onclick={() => openUrl('https://github.com/C5H12O5/TextGO')}
      >
        <GitHub class="size-5" />
      </button>
    {/snippet}
  </Title>
  <div bind:this={sentinelElement}></div>
  <div class="p-2 pt-0.5">
    {@render children()}
  </div>
</main>
