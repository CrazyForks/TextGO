<script lang="ts" module>
  import type { Prompt } from '$lib/types';

  /**
   * 提示词模板
   */
  const PROMPT_PLACEHOLDER = `
提示词中可使用以下变量:
{{selection}} - 选中的文本
{{clipboard}} - 剪贴板文本
{{datetime}}  - 当前日期时间 (ISO 8601格式)
`.trimStart();
</script>

<script lang="ts">
  import { enhance } from '$app/forms';
  import { CodeMirror, Label, Modal, alert } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { Loading } from '$lib/states.svelte';
  import { markdown } from '@codemirror/lang-markdown';
  import { At, HeadCircuit, Lightbulb } from 'phosphor-svelte';
  import { tick } from 'svelte';

  const { prompts }: { prompts: Prompt[] } = $props();
  const schema = buildFormSchema(({ text }) => ({ name: text().maxlength(64) }));
  const loading = new Loading();

  let promptId: string = $state('');
  let promptName: string = $state('');
  let promptText: string = $state('');
  let systemPromptText: string = $state('');

  let promptModal: Modal;
  let nameInputElement: HTMLInputElement;
  let collapseCheckbox: HTMLInputElement;

  export const showModal = (id?: string) => {
    if (id) {
      const prompt = prompts.find((p) => p.id === id);
      if (prompt) {
        promptId = id;
        promptName = prompt.id;
        promptText = prompt.prompt;
        systemPromptText = prompt.systemPrompt || '';
      }
    }
    promptModal.show();
    // 模态框完全显示后设置焦点
    tick().then(() => {
      collapseCheckbox?.blur();
      nameInputElement?.focus();
    });
  };

  /**
   * 保存提示词到本地存储
   *
   * @param form - 表单元素
   */
  function save(form: HTMLFormElement) {
    promptName = promptName.trim();
    const prompt = prompts.find((p) => p.id === promptName);
    if (prompt && prompt.id !== promptId) {
      alert({ level: 'error', message: '该名称已被使用' });
      const nameInput = form.querySelector('input[name="name"]');
      (nameInput as HTMLInputElement | null)?.focus();
      return;
    }
    if (!promptText || promptText.trim().length === 0) {
      alert({ level: 'error', message: '提示词内容不能为空' });
      return;
    }
    loading.start();
    if (prompt) {
      // 更新提示词
      prompt.prompt = promptText;
      prompt.systemPrompt = systemPromptText;
      alert('提示词更新成功');
    } else {
      // 新增提示词
      prompts.push({
        id: promptName,
        prompt: promptText,
        systemPrompt: systemPromptText
      });
      // 重置表单
      promptName = '';
      promptText = '';
      systemPromptText = '';
      alert('提示词添加成功');
    }
    promptModal.close();
    loading.end();
  }
</script>

<Modal icon={Lightbulb} title="{promptId ? '更新' : '新增'}提示词" bind:this={promptModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      save(formElement);
    }}
  >
    <fieldset class="fieldset">
      <Label required>名称</Label>
      <label class="input w-full">
        <At class="size-4 opacity-50" />
        <input
          class="grow"
          {...schema.name}
          bind:value={promptName}
          bind:this={nameInputElement}
          disabled={!!promptId}
        />
      </label>
      <Label required tip="明确描述您希望 AI 执行的具体任务或目标">提示词</Label>
      <CodeMirror language={markdown()} title="提示词" placeholder={PROMPT_PLACEHOLDER} bind:document={promptText} />
      <div class="collapse-arrow collapse mt-2 border">
        <input type="checkbox" class="peer" bind:this={collapseCheckbox} />
        <div class="collapse-title border-b-transparent transition-all duration-200 peer-checked:border-b">
          <HeadCircuit class="size-5" />
          系统提示词 (可选)
        </div>
        <div class="collapse-content overflow-x-auto !p-0">
          <CodeMirror
            language={markdown()}
            title="系统提示词"
            class="rounded-t-none border-x-0 border-b-0"
            bind:document={systemPromptText}
          />
        </div>
      </div>
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => promptModal.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        确 定
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
