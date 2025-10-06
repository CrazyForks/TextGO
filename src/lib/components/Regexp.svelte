<script lang="ts">
  import { enhance } from '$app/forms';
  import { Label, Modal, alert } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { Loading } from '$lib/states.svelte';
  import type { Regexp } from '$lib/types';
  import { FingerprintSimple, Hash } from 'phosphor-svelte';

  const { regexps }: { regexps: Regexp[] } = $props();
  const schema = buildFormSchema(({ text }) => ({
    name: text().maxlength(32),
    pattern: text().minlength(1).maxlength(128)
  }));
  const loading = new Loading();

  let regexpId: string = $state('');
  let regexpName: string = $state('');
  let regexpPattern: string = $state('');

  let regexpModal: Modal;
  export const showModal = (id?: string) => {
    if (id) {
      const regexp = regexps.find((p) => p.id === id);
      if (regexp) {
        regexpId = id;
        regexpName = regexp.id;
        regexpPattern = regexp.pattern;
      }
    }
    regexpModal.show();
  };

  /**
   * 保存正则表达式到本地存储
   *
   * @param form - 表单元素
   */
  function save(form: HTMLFormElement) {
    regexpName = regexpName.trim();
    const regexp = regexps.find((p) => p.id === regexpName);
    if (regexp && regexp.id !== regexpId) {
      alert({ level: 'error', message: '该名称已被使用' });
      const nameInput = form.querySelector('input[name="name"]');
      (nameInput as HTMLInputElement | null)?.focus();
      return;
    }
    loading.start();
    if (regexp) {
      // 更新正则表达式
      regexp.pattern = regexpPattern;
      alert('正则表达式更新成功');
      loading.end();
    } else {
      // 新增正则表达式
      regexps.push({
        id: regexpName,
        pattern: regexpPattern
      });
      // 重置表单
      regexpName = '';
      regexpPattern = '';
      alert('正则表达式添加成功');
    }
    regexpModal.close();
    loading.end();
  }
</script>

<Modal icon={FingerprintSimple} title="{regexpId ? '更新' : '新增'}分类模型" bind:this={regexpModal}>
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
        <Hash class="size-4 opacity-50" />
        <input class="autofocus grow" {...schema.name} bind:value={regexpName} disabled={!!regexpId} />
      </label>
      <Label required>正则表达式</Label>
      <label class="input w-full">
        <Hash class="size-4 opacity-50" />
        <input class="autofocus grow" {...schema.pattern} bind:value={regexpPattern} />
      </label>
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => regexpModal.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        确 定
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
