import { messages } from '$lib/components/Confirm.svelte';
import { modals } from '$lib/components/Modal.svelte';
import { getLocale } from '$lib/paraglide/runtime';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { ActionReturn } from 'svelte/action';
import type { Instance, Props } from 'tippy.js';
import tippy from 'tippy.js';

/**
 * Create tooltip using Tippy.js
 *
 * @param target - target element
 * @param props - tooltip properties
 * @returns svelte action return value
 */
export function tooltip(target: HTMLElement, props: Partial<Props>): ActionReturn<Partial<Props>> {
  let instance: Instance | null = null;
  if (props && props.content) {
    // check if target element is inside dialog element
    let el: HTMLElement | null = target;
    while (el && el.nodeName !== 'DIALOG') {
      el = el.parentElement;
    }
    const dialog = el as HTMLDialogElement | null;
    if (dialog) {
      // set appendTo property to dialog element
      props.appendTo = dialog;
    }
    // create tooltip instance
    instance = tippy(target, props);
  }
  return {
    update: (props) => {
      if (props && props.content) {
        if (instance) {
          instance.setProps(props);
        } else {
          instance = tippy(target, props);
        }
      } else if (instance) {
        instance.destroy();
        instance = null;
      }
    },
    destroy: () => {
      if (instance) {
        instance.destroy();
      }
    }
  };
}

/**
 * Format ISO8601 datetime string
 *
 * @param str - ISO8601 format datetime string
 * @returns formatted datetime string
 */
export function formatISO8601(str: string | null | undefined): string {
  if (!str) {
    return '';
  }
  const datetime = new Date(str);
  return datetime.toLocaleString(getLocale(), {
    dateStyle: 'medium',
    timeStyle: 'medium'
  });
}

/**
 * Freeze current window size to prevent blank edges when rapidly resizing window
 * https://github.com/tauri-apps/tauri/issues/6322
 */
export function freeze(): Promise<void> {
  return getCurrentWindow().setResizable(false);
}

/**
 * Unfreeze window size when there are no message boxes and modal dialogs
 */
export function unfreeze(): Promise<void> {
  if (messages.size === 0 && modals.size === 0) {
    return getCurrentWindow().setResizable(true);
  }
  return Promise.reject();
}
