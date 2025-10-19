<script lang="ts">
  import { enhance } from '$app/forms';
  import { alert, Label, Modal, Select } from '$lib/components';
  import { MODEL_MARK, PROMPT_MARK, REGEXP_MARK, SCRIPT_MARK } from '$lib/constants';
  import { manager } from '$lib/manager';
  import { BUILTIN_CASES, NATURAL_CASES, PROGRAM_CASES } from '$lib/matcher';
  import { Loading } from '$lib/states.svelte';
  import { models, prompts, regexps, scripts, shortcuts } from '$lib/stores.svelte';
  import type { Option, Rule } from '$lib/types';
  import { ArrowFatLineRight, FingerprintSimple, Sparkle } from 'phosphor-svelte';

  // 加载状态
  const loading = new Loading();

  // 要绑定的键位
  let lastKey: string = $state('');

  // 文本类型
  let textCase: string = $state('');

  // 动作标识
  let actionId: string = $state('');

  // 规则模态框
  let ruleModal: Modal;
  export const showModal = (key: string) => {
    lastKey = key.toUpperCase();
    ruleModal.show();
  };

  // 文本类型选项
  const textCases: Option[] = $derived.by(() => {
    const options: Option[] = [{ value: '', label: '跳过' }];
    if (models.current && models.current.length > 0) {
      options.push({ value: '--model--', label: '-- 分类模型 --', disabled: true });
      for (const m of models.current) {
        options.push({ value: MODEL_MARK + m.id, label: m.id });
      }
    }
    if (regexps.current && regexps.current.length > 0) {
      options.push({ value: '--regexp--', label: '-- 正则表达式 --', disabled: true });
      for (const r of regexps.current) {
        options.push({ value: REGEXP_MARK + r.id, label: r.id });
      }
    }
    options.push(
      ...[
        { value: '--builtin--', label: '-- 常规 --', disabled: true },
        ...BUILTIN_CASES,
        { value: '--natural--', label: '-- 自然语言 --', disabled: true },
        ...NATURAL_CASES,
        { value: '--program--', label: '-- 编程语言 --', disabled: true },
        ...PROGRAM_CASES
      ]
    );
    return options;
  });

  // 动作标识选项
  const actionIds: Option[] = $derived.by(() => {
    const options: Option[] = [{ value: '', label: '显示主窗口' }];
    if (scripts.current && scripts.current.length > 0) {
      options.push({ value: '--script--', label: '-- 脚本 --', disabled: true });
      for (const s of scripts.current) {
        options.push({ value: SCRIPT_MARK + s.id, label: s.id });
      }
    }
    if (prompts.current && prompts.current.length > 0) {
      options.push({ value: '--prompt--', label: '-- 对话 --', disabled: true });
      for (const p of prompts.current) {
        options.push({ value: PROMPT_MARK + p.id, label: p.id });
      }
    }
    return options;
  });

  /**
   * 注册并保存规则
   *
   * @param form - 表单元素
   */
  async function register(form: HTMLFormElement) {
    const rules = shortcuts.current[lastKey];
    if (rules.find((r) => r.key === lastKey && r.case === textCase)) {
      alert({ level: 'error', message: '该类型已被使用' });
      return;
    }
    loading.start();
    try {
      await manager.register({
        id: crypto.randomUUID(),
        key: lastKey,
        case: textCase,
        action: actionId
      });
      form.reset();
      ruleModal.close();
      alert('规则添加成功');
    } catch (error) {
      console.error('规则添加失败:', error);
    } finally {
      loading.end();
    }
  }

  /**
   * 注销并删除规则
   *
   * @param rule - 规则对象
   */
  export async function unregister(rule: Rule) {
    try {
      await manager.unregister(rule);
    } catch (error) {
      console.error('规则删除失败:', error);
    }
  }

  /**
   * 获取文本类型标签
   *
   * @param value - 文本类型值
   * @returns 文本类型标签
   */
  export function getCaseLabel(value: string): string | null {
    const option = textCases.find((c) => c.value === value);
    return option ? option.label : null;
  }

  /**
   * 获取动作标识标签
   *
   * @param value - 动作标识值
   * @returns 动作标识标签
   */
  export function getActionLabel(value: string): string | null {
    const option = actionIds.find((a) => a.value === value);
    return option ? option.label : null;
  }
</script>

<Modal icon={Sparkle} title="新增规则" bind:this={ruleModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      register(formElement);
    }}
  >
    <fieldset class="fieldset">
      <Label icon={FingerprintSimple} class="mt-4">识别类型</Label>
      <Select bind:value={textCase} options={textCases} class="w-full" />
      <Label icon={ArrowFatLineRight} class="mt-4">执行动作</Label>
      <Select bind:value={actionId} options={actionIds} class="w-full" />
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => ruleModal?.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        确 定
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
