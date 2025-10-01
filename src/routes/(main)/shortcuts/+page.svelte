<script lang="ts">
  import { enhance } from '$app/forms';
  import { alert, Button, confirm, Hotkey, List, Modal, Shortcut } from '$lib/components';
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
    Info,
    Robot,
    Sparkle,
    StackPlus,
    Trash,
    Warning
  } from 'phosphor-svelte';
  import { onMount, tick } from 'svelte';
  import { fly } from 'svelte/transition';

  let { data } = $props();
  let { shortcuts, models, scripts, prompts } = data;
  let totalRules = $derived(Object.values(shortcuts.current).reduce((sum, arr) => sum + arr.length, 0));

  const osType = type();

  let hotkey = $state<Hotkey>();
  let hotkeyModal: Modal;

  let key: string = $state('');

  // 表单约束
  const schema = buildFormSchema(({ text }) => ({
    key: text().maxlength(1).pattern('^[a-zA-Z0-9]$').oninvalid(oninvalid('暂不支持该键位'))
  }));

  function checkDuplicate(value: string) {
    if (value && shortcuts.current[value.toUpperCase()]) {
      oninvalid('该键位已被注册')();
      return false;
    }
    return true;
  }

  function oninvalid(message: string) {
    return () => {
      // 清除输入
      key = '';
      // 弹出提示
      alert({ level: 'error', message: message });
    };
  }

  async function submit() {
    if (!checkDuplicate(key)) {
      return;
    }
    // 只取第一个字符并转换为大写
    const newKey = key.charAt(0).toUpperCase();
    shortcuts.current[newKey] = [];
    hotkeyModal.close();
    key = '';
    alert({ message: `快捷键【${newKey}】注册成功` });
    // 等待 DOM 更新后滚动到新注册的快捷键位置
    await tick();
    const element = document.querySelector(`[data-shortcut-key="${newKey}"]`);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }

  let showNoData = $state(false);

  onMount(() => {
    // 延迟100ms显示
    setTimeout(() => {
      showNoData = true;
    }, 100);
  });
</script>

<div class="relative min-h-(--app-h) rounded-container">
  <div class="flex items-center justify-between">
    <span class="pl-1 text-sm opacity-60">
      已注册快捷键数量: {Object.keys(shortcuts.current).length}
      {#if totalRules > 0}
        <span class="text-xs tracking-wider opacity-50">({totalRules}条规则)</span>
      {/if}
    </span>
    <button class="btn text-sm btn-sm btn-submit" onclick={() => hotkeyModal.show()}>
      <StackPlus class="size-5" />注册快捷键
    </button>
  </div>
  {#if showNoData && Object.keys(shortcuts.current).length === 0}
    <div class="pointer-events-none absolute inset-0 flex items-center justify-center">
      <NoData class="m-auto size-64 pl-4 opacity-10" />
    </div>
  {/if}
  {#each Object.keys(shortcuts.current).sort() as key (key)}
    <div data-shortcut-key={key} in:fly={{ x: -100, duration: 200 }} out:fly={{ x: 100, duration: 200 }}>
      <div class="flex items-center justify-between pt-8 pb-2">
        <Shortcut {key} />
        <Button
          icon={Trash}
          onclick={() => {
            const clear = () => {
              for (const item of shortcuts.current[key] || []) {
                hotkey?.unregister(item);
              }
              delete shortcuts.current[key];
            };
            // 规则为空时直接删除，否则需要确认
            if (shortcuts.current[key].length > 0) {
              confirm({
                title: `删除快捷键[${key}]`,
                message: '数据删除后不可恢复，是否继续？',
                onconfirm: clear
              });
            } else {
              clear();
            }
          }}
        ></Button>
      </div>
      <List
        hint="选中文本后按下快捷键触发动作"
        bind:data={shortcuts.current[key]}
        oncreate={() => hotkey?.showModal(key)}
        ondelete={(item) => hotkey?.unregister(item)}
      >
        {#snippet title()}
          <Sparkle class="mx-1 size-4 opacity-60" />
          <span class="text-sm tracking-wide opacity-60">
            {#if shortcuts.current[key].length > 0}
              {shortcuts.current[key].length} 条规则
            {:else}
              暂无规则
            {/if}
          </span>
        {/snippet}
        {#snippet row(item)}
          {@const caseLabel = hotkey?.getCaseLabel(item.case)}
          {@const actionLabel = hotkey?.getActionLabel(item.action)}
          <div class="ml-4 flex w-48 items-center gap-1 truncate" title={caseLabel}>
            {#if item.case === ''}
              <span class="truncate opacity-30">{caseLabel}</span>
            {:else if caseLabel}
              <FingerprintSimple class="size-4 shrink-0 opacity-60" />
              <span class="truncate">{caseLabel}</span>
            {:else}
              <Warning class="size-4 shrink-0 opacity-50" />
              <span class="truncate opacity-50">类型已失效</span>
            {/if}
          </div>
          <ArrowFatLineRight class="size-4 shrink-0 opacity-15" />
          <div class="list-col-grow ml-4 flex items-center gap-1 truncate" title={actionLabel}>
            {#if item.action === ''}
              <span class="truncate opacity-30">{actionLabel}</span>
            {:else if actionLabel}
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
          </div>
        {/snippet}
      </List>
    </div>
  {/each}
</div>

<Modal maxWidth="20rem" icon={StackPlus} title="注册快捷键" bind:this={hotkeyModal}>
  <form
    method="post"
    use:enhance={({ cancel }) => {
      cancel();
      submit();
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
          class="autofocus input h-10 w-12 text-xl"
          {...schema.key}
          bind:value={key}
          oninput={(event) => (event.target as HTMLInputElement)?.form?.requestSubmit()}
        />
      </div>
      <div class="flex items-center justify-center gap-1 text-xs tracking-wider opacity-30">
        <Info class="size-4" />请输入要注册的字母或数字键
      </div>
    </fieldset>
  </form>
</Modal>

<Hotkey bind:this={hotkey} models={models.current} scripts={scripts.current} prompts={prompts.current} />
