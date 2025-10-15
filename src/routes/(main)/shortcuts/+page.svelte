<script lang="ts">
  import { enhance } from '$app/forms';
  import { alert, Button, confirm, Hotkey, List, Modal, Shortcut } from '$lib/components';
  import { PROMPT_MARK, SCRIPT_MARK } from '$lib/constants';
  import { buildFormSchema } from '$lib/constraint';
  import { JavaScript, LMStudio, NoData, Ollama, Python } from '$lib/icons';
  import { type } from '@tauri-apps/plugin-os';
  import {
    ArrowFatLineRight,
    ArrowFatUp,
    Command,
    Control,
    FingerprintSimple,
    Info,
    Sparkle,
    StackPlus,
    Trash,
    Warning
  } from 'phosphor-svelte';
  import { onMount, tick } from 'svelte';
  import { fly } from 'svelte/transition';

  let { data } = $props();
  let { shortcuts, models, scripts, prompts } = data;

  let key: string = $state('');
  let keyModal: Modal;

  let hotkey = $state<Hotkey>();

  // 表单约束
  const schema = buildFormSchema(({ text }) => ({
    key: text().maxlength(1).pattern('^[a-zA-Z0-9]$').oninvalid(oninvalid('暂不支持该键位'))
  }));

  // 操作系统类型
  const osType = type();

  // 计算总规则数
  let totalRules = $derived(Object.values(shortcuts.current).reduce((sum, arr) => sum + arr.length, 0));

  // 是否处于输入法组合状态
  let compositing: boolean = false;

  /**
   * 输入无效时的处理
   * @param message - 提示信息
   */
  function oninvalid(message: string) {
    return () => {
      // 清除输入
      key = '';
      // 弹出提示
      alert({ level: 'error', message: message });
    };
  }

  /**
   * 检查是否重复
   * @param value - 输入值
   */
  function checkDuplicate(value: string) {
    if (value && shortcuts.current[value.toUpperCase()]) {
      oninvalid('该键位已被注册')();
      return false;
    }
    return true;
  }

  /**
   * 提交注册
   */
  async function submit() {
    // 只取第一个字符并转换为大写
    const newKey = key.charAt(0).toUpperCase();
    if (!checkDuplicate(newKey)) {
      return;
    }
    shortcuts.current[newKey] = [];
    keyModal.close();
    key = '';
    alert({ message: `快捷键组【 ${newKey} 】注册成功` });
    // 等待 DOM 更新后滚动到新注册的快捷键位置
    await tick();
    const element = document.querySelector(`[data-shortcut-key="${newKey}"]`);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }

  /**
   * 根据动作ID获取脚本
   *
   * @param action - 动作ID
   */
  function getScript(action: string) {
    const id = action.replace(SCRIPT_MARK, '');
    return scripts.current.find((item) => item.id === id);
  }

  /**
   * 根据动作ID获取提示词
   *
   * @param action - 动作ID
   */
  function getPrompt(action: string) {
    const id = action.replace(PROMPT_MARK, '');
    return prompts.current.find((item) => item.id === id);
  }

  // 控制无数据时的显示延迟，避免闪烁
  let showNoData = $state(false);
  onMount(() => {
    setTimeout(() => {
      showNoData = true;
    }, 100);
  });
</script>

<div class="relative min-h-(--app-h) rounded-container">
  <div class="flex items-center justify-between">
    <span class="pl-1 text-sm opacity-60">
      已注册快捷键组数量: {Object.keys(shortcuts.current).length}
      {#if totalRules > 0}
        <span class="text-xs tracking-wider opacity-50">({totalRules}条规则)</span>
      {/if}
    </span>
    <button class="btn text-sm btn-sm btn-submit" onclick={() => keyModal.show()}>
      <StackPlus class="size-5" />注册快捷键组
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
                title: `删除快捷键组[${key}]`,
                message: '数据删除后无法恢复，是否继续？',
                onconfirm: clear
              });
            } else {
              clear();
            }
          }}
        ></Button>
      </div>
      <List
        name="规则"
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
          <div class="ml-4 flex w-60 items-center gap-1 truncate" title={caseLabel}>
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
                {@const script = getScript(item.action)}
                {#if script?.lang === 'javascript'}
                  <JavaScript class="h-5 shrink-0" />
                {:else if script?.lang === 'python'}
                  <Python class="h-5 shrink-0" />
                {/if}
              {:else if item.action.startsWith(PROMPT_MARK)}
                {@const prompt = getPrompt(item.action)}
                {#if prompt?.provider === 'ollama'}
                  <Ollama class="h-5 shrink-0" />
                {:else if prompt?.provider === 'lmstudio'}
                  <LMStudio class="h-5 shrink-0" />
                {/if}
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

<Modal maxWidth="20rem" icon={StackPlus} title="注册快捷键组" bind:this={keyModal}>
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
          oninput={(event) => !compositing && (event.target as HTMLInputElement)?.form?.requestSubmit()}
          oncompositionstart={() => (compositing = true)}
          oncompositionend={(event) => (
            (compositing = false),
            (event.target as HTMLInputElement)?.form?.requestSubmit()
          )}
        />
      </div>
      <div class="flex items-center justify-center gap-1 text-xs tracking-wider opacity-30">
        <Info class="size-4" />请输入要注册的字母或数字键
      </div>
    </fieldset>
  </form>
</Modal>

<Hotkey bind:this={hotkey} models={models.current} scripts={scripts.current} prompts={prompts.current} />
