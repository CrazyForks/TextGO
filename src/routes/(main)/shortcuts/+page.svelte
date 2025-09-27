<script lang="ts">
  import { enhance } from '$app/forms';
  import { Button, Hotkey, List, Modal, Shortcut } from '$lib/components';
  import { PROMPT_MARK, SCRIPT_MARK } from '$lib/constants';
  import { buildFormSchema } from '$lib/constraint';
  import { NoData } from '$lib/icons';
  import { type } from '@tauri-apps/plugin-os';
  import {
    ArrowFatLineRight,
    ArrowFatUp,
    Code,
    Command,
    Control,
    FingerprintSimple,
    Robot,
    Sparkle,
    StackPlus,
    Trash,
    Warning
  } from 'phosphor-svelte';

  let { data } = $props();
  let { shortcuts, models, scripts, prompts } = data;
  let totalRules = $derived(Object.values(shortcuts.current).reduce((sum, arr) => sum + arr.length, 0));

  const osType = type();

  let hotkey = $state<Hotkey>();
  let hotkeyModal: Modal;

  let key: string = $state('');
  // 表单约束
  const schema = buildFormSchema(({ text }) => ({
    key: text().maxlength(1).pattern('^[a-zA-Z0-9]$').oninvalid('请输入单个字母或数字')
  }));
</script>

<div class="relative min-h-(--app-h) rounded-container">
  <div class="flex items-center justify-between">
    <span class="pl-1 text-sm opacity-60">
      已注册快捷键数量: {Object.keys(shortcuts.current).length}
      {#if totalRules > 0}
        <span class="text-xs opacity-50">({totalRules}条规则)</span>
      {/if}
    </span>
    <button class="btn text-sm btn-sm btn-submit" onclick={() => hotkeyModal.show()}>
      <StackPlus class="size-5" />注册快捷键
    </button>
  </div>
  {#if Object.keys(shortcuts.current).length === 0}
    <div class="pointer-events-none absolute inset-0 flex items-center justify-center">
      <NoData class="m-auto size-64 pl-4 opacity-10" />
    </div>
  {/if}
  {#each Object.keys(shortcuts.current) as key}
    <div class="flex items-center justify-between pt-8 pb-2">
      <Shortcut {key} />
      <Button
        icon={Trash}
        onclick={() => {
          for (const item of shortcuts.current[key] || []) {
            hotkey?.unregister(item);
          }
          delete shortcuts.current[key];
        }}
      ></Button>
    </div>
    <List
      hint="暂无规则"
      bind:data={shortcuts.current[key]}
      oncreate={() => hotkey?.showModal(key)}
      ondelete={(item) => hotkey?.unregister(item)}
    >
      {#snippet title()}
        {#if shortcuts.current[key].length > 0}
          <Sparkle class="mx-1 size-4 opacity-60" />
          <span class="text-sm tracking-wide opacity-60">{shortcuts.current[key].length} 条规则</span>
        {/if}
      {/snippet}
      {#snippet row(item)}
        {@const caseLabel = hotkey?.getCaseLabel(item.case)}
        {@const actionLabel = hotkey?.getActionLabel(item.action)}
        <span class="list-col-grow ml-4 flex items-center gap-1 truncate" title={caseLabel}>
          {#if caseLabel}
            <FingerprintSimple class="size-4 shrink-0 opacity-50" />
            <span class="truncate">{caseLabel}</span>
          {:else if caseLabel === ''}
            <span class="truncate opacity-30">跳过</span>
          {:else}
            <Warning class="size-4 shrink-0 opacity-50" />
            <span class="truncate opacity-50">类型已失效</span>
          {/if}
          <ArrowFatLineRight class="ml-auto size-4 shrink-0 opacity-50" />
        </span>
        <span class="flex w-38 items-center gap-1" title={actionLabel}>
          {#if actionLabel}
            {#if item.action.startsWith(SCRIPT_MARK)}
              <Code class="size-4 shrink-0" />
            {:else if item.action.startsWith(PROMPT_MARK)}
              <Robot class="size-4 shrink-0" />
            {/if}
            <span class="truncate">{actionLabel}</span>
          {:else}
            <Warning class="size-4 shrink-0 opacity-50" />
            <span class="truncate opacity-50">动作已失效</span>
          {/if}
        </span>
      {/snippet}
    </List>
  {/each}
</div>

<Modal maxWidth="20rem" icon={StackPlus} title="注册快捷键" bind:this={hotkeyModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      shortcuts.current[key.toUpperCase()] = [];
      hotkeyModal.close();
      formElement.reset();
    }}
  >
    <fieldset class="fieldset">
      <div class="flex items-center justify-center gap-4 py-2">
        <kbd class="kbd h-10 w-12">
          {#if osType === 'macos'}
            <Command class="size-6" />
          {:else}
            <Control class="size-6" />
          {/if}
        </kbd>
        <span class="text-2xl font-bold opacity-50">+</span>
        <kbd class="kbd h-10 w-12"><ArrowFatUp class="size-6" /></kbd>
        <span class="text-2xl font-bold opacity-50">+</span>
        <input
          class="input h-10 w-12 text-xl"
          {...schema.key}
          bind:value={key}
          oninput={(event) => (event.target as HTMLInputElement)?.form?.requestSubmit()}
        />
      </div>
    </fieldset>
  </form>
</Modal>

<Hotkey bind:this={hotkey} models={models.current} scripts={scripts.current} prompts={prompts.current} />
