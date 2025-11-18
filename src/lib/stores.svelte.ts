import { manager } from '$lib/manager';
import type { Entry, Rule, Model, Prompt, Regexp, Script } from '$lib/types';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LazyStore } from '@tauri-apps/plugin-store';
import { untrack } from 'svelte';

// create a global LazyStore instance
const store = new LazyStore('.settings.dat');

/**
 * Options for creating persisted state.
 */
type Options<T> = {
  /** Callback function when loading is complete. */
  onload?: (value: T) => void;
  /** Callback function when the stored value changes. */
  onchange?: (value: T | $state.Snapshot<T>) => void;
};

/**
 * Create a persisted reactive state.
 *
 * @param key - key for local storage
 * @param initial - initial value
 * @param options - persistence options
 * @returns persisted state object
 */
function persisted<T>(key: string, initial: T, options?: Options<T>) {
  let state = $state(initial);
  let initialized = $state(false);

  // load data from store
  store
    .get<T>(key)
    .then((item) => {
      if (item !== undefined) {
        untrack(() => {
          state = item;
        });
        options?.onload?.(item);
        options?.onchange?.(item);
      }
      initialized = true;
    })
    .catch((error) => {
      console.error(`Failed to load persisted data: ${error}`);
      initialized = true;
    });

  // save data to store
  $effect.root(() => {
    $effect(() => {
      if (!initialized) {
        return;
      }
      // get snapshot of current state
      const snapshot = $state.snapshot(state);
      store
        .set(key, snapshot)
        .then(() => {
          // save to local file
          store.save();
        })
        .then(() => {
          options?.onchange?.(snapshot);
        })
        .catch((error) => {
          console.error(`Failed to save persisted data: ${error}`);
        });

      // sync to localStorage
      if (snapshot) {
        localStorage.setItem(key, JSON.stringify(snapshot));
      } else {
        localStorage.removeItem(key);
      }
    });
    // ensure it won't be cleaned up
    return () => {};
  });

  // listen for localStorage changes to implement cross-window sync
  window.addEventListener('storage', (event) => {
    if (event.key === key) {
      if (event.newValue) {
        state = JSON.parse(event.newValue);
      } else {
        state = initial;
      }
    }
  });

  return {
    get current() {
      return state;
    },
    set current(value: T) {
      state = value;
    }
  };
}

// theme (light / dark)
export const theme = persisted<string>('theme', 'light', {
  onchange: (theme) => {
    // set data-theme attribute on root element to switch theme
    const root = document.documentElement;
    root.setAttribute('data-theme', theme);
    // the theme set here is application-wide, not specific to the current window
    getCurrentWindow().setTheme(theme === 'light' ? 'light' : 'dark');
  }
});

// auto start setting
export const autoStart = persisted<boolean>('autoStart', false);

// minimize to tray setting
export const minimizeToTray = persisted<boolean>('minimizeToTray', true);

// Node.js path
export const nodePath = persisted<string>('nodePath', '');

// Python path
export const pythonPath = persisted<string>('pythonPath', '');

// Ollama service address
export const ollamaHost = persisted<string>('ollamaHost', '');

// number of history records to retain
export const historySize = persisted<number>('historySize', 5);

// shortcut group
export const shortcuts = persisted<Record<string, Rule[]>>(
  'shortcuts',
  {},
  {
    onload: async (shortcuts) => {
      // register all shortcut groups when main window initializes
      if (getCurrentWindow().label === 'main') {
        for (const rule of Object.values(shortcuts).flat()) {
          await manager.register(rule);
        }
      }
    }
  }
);

// classification model
export const models = persisted<Model[]>('models', []);

// regular expression
export const regexps = persisted<Regexp[]>('regexps', []);

// script
export const scripts = persisted<Script[]>('scripts', []);

// prompt
export const prompts = persisted<Prompt[]>('prompts', []);

// trigger record
export const entries = persisted<Entry[]>('entries', []);
