<script lang="ts">
  import { enhance } from '$app/forms';
  import { SingleClassTextClassifier } from '$lib/classifier';
  import { CodeMirror, Label, Modal, alert } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { Loading } from '$lib/states.svelte';
  import type { Model } from '$lib/types';
  import { FingerprintSimple, Hash } from 'phosphor-svelte';

  const { models }: { models: Model[] } = $props();
  const schema = buildFormSchema(({ text, number }) => ({
    name: text().maxlength(32),
    threshold: number().min(0.01).max(0.99).step(0.01)
  }));
  const loading = new Loading();

  let modelId: string = $state('');
  let modelName: string = $state('');
  let modelSample: string = $state('');
  let modelThreshold: number = $state(0.5);

  let modelModal: Modal;
  export const showModal = (id?: string) => {
    if (loading.started) {
      alert({ level: 'error', message: `【${modelName}】模型训练中，请稍后` });
      return;
    }
    if (id) {
      const model = models.find((p) => p.id === id);
      if (model) {
        modelId = id;
        modelName = model.id;
        modelSample = model.sample;
        modelThreshold = model.threshold;
      }
    }
    modelModal.show();
  };

  /**
   * 保存模型信息到本地存储
   *
   * @param form - 表单元素
   */
  function save(form: HTMLFormElement) {
    modelName = modelName.trim();
    const model = models.find((p) => p.id === modelName);
    if (model && model.id !== modelId) {
      alert({ level: 'error', message: '该名称已被使用' });
      const nameInput = form.querySelector('input[name="name"]');
      (nameInput as HTMLInputElement | null)?.focus();
      return;
    }
    if (!SingleClassTextClassifier.validateTrainingData(modelSample)) {
      alert({ level: 'error', message: '训练数据格式无效或没有有效样本' });
      return;
    }
    loading.start();
    if (model) {
      // 更新模型信息
      model.threshold = modelThreshold;
      alert('模型信息更新成功');
      loading.end();
    } else {
      // 训练分类模型
      const id = modelName;
      models.push({
        id: id,
        sample: modelSample,
        threshold: modelThreshold
      });
      alert(`【${modelName}】模型训练开始`);
      // 训练模型
      const classifier = new SingleClassTextClassifier(modelName);
      classifier
        .trainModel(modelSample)
        .then(() => {
          const model = models.find((c) => c.id === id);
          if (model) {
            model.modelTrained = true;
          }
          loading.end();
          alert(`【${modelName}】模型训练成功`);
          // 重置表单
          modelName = '';
          modelSample = '';
          modelThreshold = 0.5;
        })
        .catch((error) => {
          console.error(error);
          const model = models.find((c) => c.id === id);
          if (model) {
            model.modelTrained = false;
          }
          loading.end();
          alert({ level: 'error', message: `【${modelName}】模型训练失败` });
        });
    }
    modelModal.close();
  }
</script>

<Modal icon={FingerprintSimple} title="{modelId ? '更新' : '新增'}分类模型" bind:this={modelModal}>
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
        <input class="autofocus grow" {...schema.name} bind:value={modelName} disabled={!!modelId} />
      </label>
      <Label required tip="根据提供的正向样本数据训练文本分类模型">正向样本</Label>
      <CodeMirror
        title="正向样本"
        darkMode={false}
        bind:document={modelSample}
        readOnly={!!modelId}
        placeholder="请输入至少 3 条正向样本，每行一条"
      />
      <Label required tip="取值范围 0.01 ~ 0.99">置信度阈值</Label>
      <label class="input input-sm w-full">
        <input class="grow" {...schema.threshold} bind:value={modelThreshold} />
      </label>
    </fieldset>
    <div class="modal-action">
      <button type="button" class="btn" onclick={() => modelModal.close()}>取 消</button>
      <button type="submit" class="btn btn-submit" disabled={loading.started}>
        {modelId ? '更 新' : '训 练'}
        {#if loading.delayed}
          <span class="loading loading-xs loading-dots"></span>
        {/if}
      </button>
    </div>
  </form>
</Modal>
