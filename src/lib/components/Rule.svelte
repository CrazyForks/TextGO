<script lang="ts">
  import { enhance } from '$app/forms';
  import { alert, Label, Modal, Select } from '$lib/components';
  import { MODEL_MARK, PROMPT_MARK, REGEXP_MARK, SCRIPT_MARK } from '$lib/constants';
  import { manager } from '$lib/manager';
  import { BUILTIN_CASES, NATURAL_CASES, PROGRAM_CASES } from '$lib/matcher';
  import { m } from '$lib/paraglide/messages';
  import { Loading } from '$lib/states.svelte';
  import { models, prompts, regexps, scripts, shortcuts } from '$lib/stores.svelte';
  import type { Option, Rule } from '$lib/types';
  import { ArrowFatLineRight, Code, FingerprintSimple, Sparkle, Translate } from 'phosphor-svelte';

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
    const options: Option[] = [{ value: '', label: m.skip() }];
    if (models.current && models.current.length > 0) {
      options.push({ value: '--model--', label: `-- ${m.classification_model()} --`, disabled: true });
      for (const model of models.current) {
        options.push({ value: MODEL_MARK + model.id, label: model.id });
      }
    }
    if (regexps.current && regexps.current.length > 0) {
      options.push({ value: '--regexp--', label: `-- ${m.regexp()} --`, disabled: true });
      for (const regexp of regexps.current) {
        options.push({ value: REGEXP_MARK + regexp.id, label: regexp.id });
      }
    }
    options.push(
      ...[
        { value: '--builtin--', label: `-- ${m.general()} --`, disabled: true },
        ...BUILTIN_CASES,
        { value: '--natural--', label: `-- ${m.natural_language()} --`, disabled: true },
        ...NATURAL_CASES.map((c) => ({ ...c, icon: Translate })),
        { value: '--program--', label: `-- ${m.programming_language()} --`, disabled: true },
        ...PROGRAM_CASES.map((c) => ({ ...c, icon: Code }))
      ]
    );
    return options;
  });

  // 动作标识选项
  const actionIds: Option[] = $derived.by(() => {
    const options: Option[] = [{ value: '', label: m.show_main_window() }];
    if (scripts.current && scripts.current.length > 0) {
      options.push({ value: '--script--', label: `-- ${m.script()} --`, disabled: true });
      for (const script of scripts.current) {
        options.push({ value: SCRIPT_MARK + script.id, label: script.id });
      }
    }
    if (prompts.current && prompts.current.length > 0) {
      options.push({ value: '--prompt--', label: `-- ${m.conversation()} --`, disabled: true });
      for (const prompt of prompts.current) {
        options.push({ value: PROMPT_MARK + prompt.id, label: prompt.id });
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
      alert({ level: 'error', message: m.type_already_used() });
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
      alert(m.rule_added_success());
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
   * 获取文本类型选项
   *
   * @param value - 文本类型值
   * @returns 文本类型选项
   */
  export function getCaseOption(value: string): Option | undefined {
    return textCases.find((c) => c.value === value);
  }

  /**
   * 获取动作标识选项
   *
   * @param value - 动作标识值
   * @returns 动作标识选项
   */
  export function getActionOption(value: string): Option | undefined {
    return actionIds.find((a) => a.value === value);
  }
</script>

<Modal icon={Sparkle} title="{m.add()}{m.rule()}" bind:this={ruleModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      register(formElement);
    }}
  >
    <fieldset class="fieldset">
      <Label icon={FingerprintSimple} class="mt-4">{m.recognize_type()}</Label>
      <Select bind:value={textCase} options={textCases} class="w-full" />
      <Label icon={ArrowFatLineRight} class="mt-4">{m.execute_action()}</Label>
      <Select bind:value={actionId} options={actionIds} class="w-full" />
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => ruleModal?.close()}>{m.cancel()}</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        {m.confirm()}
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
