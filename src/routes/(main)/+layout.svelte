<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { Button, Title } from '$lib/components';
  import { GitHub, Moon, Sun } from '$lib/icons';
  import { m } from '$lib/paraglide/messages';
  import { deLocalizeHref } from '$lib/paraglide/runtime';
  import { theme } from '$lib/stores.svelte';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { type } from '@tauri-apps/plugin-os';
  import { ClockCounterClockwise, GearSix, Keyboard } from 'phosphor-svelte';
  import { onMount, type Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();

  // operating system type
  const osType = type();

  /**
   * navigation menu item
   */
  const menus = [
    { text: m.histories(), icon: ClockCounterClockwise, path: resolve('/histories') },
    { text: m.shortcuts(), icon: Keyboard, path: resolve('/shortcuts') },
    { text: m.settings(), icon: GearSix, path: resolve('/settings') }
  ];

  /**
   * title bar bottom border display status
   */
  let titleBorder = $state(false);
  let sentinelElement: HTMLElement;

  onMount(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];
        // show title bar bottom border when sentinel element is invisible
        titleBorder = !entry.isIntersecting;
      },
      {
        // 44px is the height of the Title component
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
        // prevent default validation bubble display
        event.preventDefault();
        // focus on the input box that failed validation
        if (document.activeElement !== event.target) {
          event.target.focus();
        }
      }
    };
    // listen to all validation failure events
    document.addEventListener('invalid', oninvalid, true);
    return () => {
      // clean up event listeners
      document.removeEventListener('invalid', oninvalid, true);
    };
  });

  onMount(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        // in some cases, after hiding and showing the window, an element in the page may unexpectedly get focus, which is canceled here
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
    // listen to shortcut key registration page events
    const unlisten = listen('goto-shortcuts', async () => {
      goto(resolve('/shortcuts'));
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  });
</script>

<main class="h-screen w-screen overflow-auto overscroll-none bg-base-300">
  <Title class="sticky top-0 z-101 bg-base-300/80 backdrop-blur-sm {titleBorder ? 'border-b' : ''}">
    {#snippet fallback()}
      <!-- navigation menu -->
      <span class="flex gap-2 {osType === 'macos' ? 'mx-auto pl-16' : 'mr-auto pl-2'}">
        {#each menus as menu (menu.path)}
          {@const active = deLocalizeHref(page.url.pathname).startsWith(menu.path)}
          <Button
            size="sm"
            square={false}
            icon={menu.icon}
            text={menu.text}
            weight={active ? 'fill' : 'light'}
            class="border-none transition-all hover:bg-base-100 {active
              ? 'gradient bg-base-100 shadow-around'
              : 'text-base-content/80 hover:text-base-content'}"
            onclick={() => {
              // eslint-disable-next-line svelte/no-navigation-without-resolve
              goto(menu.path);
            }}
          />
        {/each}
      </span>
      <!-- theme toggle -->
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
      <!-- gitHub link -->
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
