<script lang="ts" module>
  import type { Script } from '$lib/types';

  /**
   * JavaScript 代码模板
   */
  const JAVASCRIPT_TEMPLATE = `
/**
 * 输入数据结构:
 * data = {
 *     "selection": "",  // 选中的文本
 *     "clipboard": "",  // 剪贴板文本
 *     "datetime": ""    // 当前日期时间 (ISO 8601格式)
 * }
 *
 * @param data - 输入的数据
 * @returns 处理后的输出
 */
function process(data) {
    // write your code here

    return "";
}
`.trimStart();

  /**
   * Python 代码模板
   */
  const PYTHON_TEMPLATE = `
def process(data):
    """
    输入数据结构:
    data = {
        "selection": "",  # 选中的文本
        "clipboard": "",  # 剪贴板文本
        "datetime": ""    # 当前日期时间 (ISO 8601格式)
    }

    Args:
        data: 输入的数据

    Returns:
        处理后的输出
    """
    # write your code here

    return ""
`.trimStart();
</script>

<script lang="ts">
  import { enhance } from '$app/forms';
  import { CodeMirror, Label, Modal, Select, alert, confirm } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { Loading } from '$lib/states.svelte';
  import { javascript } from '@codemirror/lang-javascript';
  import { python } from '@codemirror/lang-python';
  import { ArrowFatLineRight, Code, Empty } from 'phosphor-svelte';

  const { scripts }: { scripts: Script[] } = $props();
  const loading = new Loading();
  const schema = buildFormSchema(({ text }) => ({ name: text().maxlength(64) }));

  let scriptId: string = $state('');
  let scriptName: string = $state('');
  let scriptLang: 'javascript' | 'python' = $state('javascript');
  let scriptText: string = $state(JAVASCRIPT_TEMPLATE);
  let quietMode: boolean = $state(false);

  let scriptModal: Modal;
  export const showModal = (id?: string) => {
    if (id) {
      const script = scripts.find((s) => s.id === id);
      if (script) {
        scriptId = id;
        scriptName = script.id;
        scriptLang = script.lang;
        scriptText = script.script;
        quietMode = script.quietMode || false;
      }
    }
    scriptModal.show();
  };

  /**
   * 保存脚本到本地存储
   *
   * @param form - 表单元素
   */
  function save(form: HTMLFormElement) {
    scriptName = scriptName.trim();
    const script = scripts.find((s) => s.id === scriptName);
    if (script && script.id !== scriptId) {
      alert({ level: 'error', message: '该名称已被使用' });
      const nameInput = form.querySelector('input[name="name"]');
      (nameInput as HTMLInputElement | null)?.focus();
      return;
    }
    loading.start();
    if (script) {
      // 更新脚本
      script.lang = scriptLang;
      script.script = scriptText;
      script.quietMode = quietMode;
      alert('脚本更新成功');
    } else {
      // 新增脚本
      scripts.push({
        id: scriptName,
        lang: scriptLang,
        script: scriptText,
        quietMode: quietMode
      });
      // 重置表单
      scriptName = '';
      scriptLang = 'javascript';
      scriptText = JAVASCRIPT_TEMPLATE;
      quietMode = false;
      alert('脚本添加成功');
    }
    scriptModal.close();
    loading.end();
  }
</script>

<Modal icon={Code} title="{scriptId ? '更新' : '新增'}脚本" bind:this={scriptModal}>
  <form
    method="post"
    use:enhance={({ formElement, cancel }) => {
      cancel();
      save(formElement);
    }}
  >
    <fieldset class="fieldset">
      <Label required>动作名称</Label>
      <label class="input w-full">
        <ArrowFatLineRight class="size-5 opacity-50" />
        <input class="autofocus grow" {...schema.name} bind:value={scriptName} disabled={!!scriptId} />
      </label>
      <Label required>脚本类型</Label>
      <Select
        value={scriptLang}
        options={[
          { value: 'javascript', label: 'JavaScript' },
          { value: 'python', label: 'Python' }
        ]}
        class="w-full"
        disabled={!!scriptId}
        onchange={(event) => {
          const target = event.currentTarget;
          const onconfirm = () => {
            scriptLang = target.value as 'javascript' | 'python';
            scriptText = scriptLang === 'python' ? PYTHON_TEMPLATE : JAVASCRIPT_TEMPLATE;
          };
          // 判断当前代码是否为模板代码
          if (scriptText === (scriptLang === 'python' ? PYTHON_TEMPLATE : JAVASCRIPT_TEMPLATE)) {
            // 直接改变类型
            onconfirm();
          } else {
            // 确认改变类型
            confirm({
              message: '更改脚本类型会重置当前代码，是否继续？',
              oncancel: () => (target.value = scriptLang),
              onconfirm: onconfirm
            });
          }
        }}
      />
      {#key scriptLang}
        <CodeMirror
          language={scriptLang === 'python' ? python() : javascript()}
          title={scriptLang === 'python' ? 'Python 脚本' : 'JavaScript 脚本'}
          class="mt-4"
          bind:document={scriptText}
        />
      {/key}
      <label class="label mt-2 justify-between rounded-box border p-2">
        <span
          class="flex items-center gap-2 text-base tracking-wider transition-colors {quietMode
            ? 'text-base-content'
            : ''}"
        >
          <Empty class="size-5" />静默模式
        </span>
        <input type="checkbox" class="checkbox" bind:checked={quietMode} />
      </label>
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => scriptModal.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        确 定
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
