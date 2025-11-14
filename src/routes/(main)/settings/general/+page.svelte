<script lang="ts">
  import { Label, Select, Setting } from '$lib/components';
  import { m } from '$lib/paraglide/messages';
  import { getLocale, setLocale, type Locale } from '$lib/paraglide/runtime';
  import { historySize, theme } from '$lib/stores.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { ClockCounterClockwise, Swatches, Translate } from 'phosphor-svelte';

  // current language
  let locale: Locale = $state(getLocale());

  // update tray menu language
  async function updateTrayMenu() {
    try {
      await invoke('setup_tray', {
        mainWindowText: m.tray_main_window(),
        shortcutsText: m.tray_shortcuts(),
        aboutText: m.tray_about(),
        quitText: m.tray_quit()
      });
    } catch (error) {
      console.error(`Failed to update tray menu language: ${error}`);
    }
  }
</script>

<div class="flex flex-col gap-2">
  <div class="flex flex-col gap-1 rounded-container">
    <fieldset class="flex items-center justify-between">
      <Label icon={Translate}>{m.language_settings()}</Label>
      <Select
        value={locale}
        options={[
          { value: 'en', label: 'English' },
          { value: 'zh-cn', label: '简体中文' }
        ]}
        class="w-36 select-sm"
        onchange={async (event) => {
          const target = event.currentTarget;
          locale = target.value as Locale;
          setLocale(locale);
          await updateTrayMenu();
        }}
      />
    </fieldset>
    <div class="divider my-0 opacity-60"></div>
    <fieldset class="flex items-center justify-between">
      <Label icon={Swatches}>{m.theme_settings()}</Label>
      <Select
        options={[
          { value: 'light', label: m.light_theme() },
          { value: 'dracula', label: m.dark_theme() }
        ]}
        bind:value={theme.current}
        class="w-36 select-sm"
      />
    </fieldset>
  </div>
  <Setting icon={ClockCounterClockwise} title={m.history_records()}>
    <fieldset class="flex items-center justify-between">
      <Label>{m.history_records_retention()}</Label>
      <Select
        options={[
          { value: 0, label: m.history_none() },
          { value: 3, label: m.history_recent_3() },
          { value: 5, label: m.history_recent_5() },
          { value: 10, label: m.history_recent_10() },
          { value: 20, label: m.history_recent_20() }
        ]}
        bind:value={historySize.current}
        class="w-36 select-sm"
      />
    </fieldset>
  </Setting>
</div>
