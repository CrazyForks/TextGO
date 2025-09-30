<script lang="ts">
  import { SingleClassTextClassifier } from '$lib/classifier';
  import { Button, Label, List, Model, Prompt, Script, Select, Setting } from '$lib/components';
  import { JavaScript, Markdown, Python, Tensorflow } from '$lib/icons';
  import {
    ArrowFatLineRight,
    ClockCounterClockwise,
    Code,
    FingerprintSimple,
    Lightbulb,
    Package,
    PencilSimpleLine,
    Scroll,
    SlidersHorizontal,
    Sphere,
    Swatches,
    TextT,
    Translate,
    Warning
  } from 'phosphor-svelte';

  let { data } = $props();
  let { theme, models, scripts, prompts } = data;

  let scriptCreator: Script;
  let scriptUpdater: Script;
  let promptCreator: Prompt;
  let promptUpdater: Prompt;
  let modelCreator: Model;
  let modelUpdater: Model;
</script>

<div class="flex flex-col gap-2">
  <Setting icon={FingerprintSimple} title="文本类型" tip="自定义可识别的文本类型">
    <List
      icon={Sphere}
      title="分类模型"
      name="分类模型"
      hint="使用分类模型识别文本类型"
      bind:data={models.current}
      oncreate={() => modelCreator.showModal()}
      ondelete={(item) => SingleClassTextClassifier.clearSavedModel(item.id)}
    >
      {#snippet row(item)}
        <Tensorflow class="h-5" />
        <span class="list-col-grow flex items-center gap-4 truncate">
          <span class="truncate text-base">{item.id}</span>
          {#if item.modelTrained === true}
            {@const { sizeKB, vocabulary } = SingleClassTextClassifier.getModelInfo(item.id)}
            <span class="badge badge-ghost badge-sm">
              <Package class="size-4 shrink-0 opacity-50" />
              {sizeKB} KB
            </span>
            <span class="badge badge-ghost badge-sm">
              <TextT class="size-4 shrink-0 opacity-50" />
              {vocabulary} 词汇
            </span>
          {/if}
        </span>
        {#if item.modelTrained === undefined}
          <span class="flex h-8 items-center gap-2 opacity-50">
            <span class="loading loading-sm loading-spinner"></span>
            训练中
          </span>
        {:else if item.modelTrained === false}
          <span class="flex h-8 items-center gap-2 opacity-50">
            <Warning class="size-4 shrink-0" />
            训练失败
          </span>
        {:else}
          <Button
            size="sm"
            icon={PencilSimpleLine}
            onclick={(event) => {
              event.stopPropagation();
              modelUpdater.showModal(item.id);
            }}
          />
        {/if}
      {/snippet}
    </List>
    <List
      icon={Scroll}
      title="正则表达式"
      name="正则表达式"
      hint="使用正则表达式匹配文本类型"
      bind:data={models.current}
      oncreate={() => modelCreator.showModal()}
      ondelete={(item) => SingleClassTextClassifier.clearSavedModel(item.id)}
    >
      {#snippet row(item)}
        <div></div>
      {/snippet}
    </List>
  </Setting>
  <Setting icon={ArrowFatLineRight} title="触发动作" tip="自定义识别文本后可执行的动作">
    <List
      icon={Code}
      title="执行脚本"
      name="脚本"
      hint="使用预设的脚本处理选中文本"
      bind:data={scripts.current}
      oncreate={() => scriptCreator.showModal()}
      moreActions={() => console.log('moreActions')}
    >
      {#snippet row(item)}
        {#if item.lang === 'javascript'}
          <JavaScript class="h-5" />
        {:else if item.lang === 'python'}
          <Python class="h-5" />
        {/if}
        <span class="list-col-grow text-base">{item.id}</span>
        <Button
          size="sm"
          icon={PencilSimpleLine}
          onclick={(event) => {
            event.stopPropagation();
            scriptUpdater.showModal(item.id);
          }}
        />
      {/snippet}
    </List>
    <List
      icon={Lightbulb}
      title="发起对话"
      name="提示词"
      hint="使用预设的提示词向 AI 模型发起对话"
      bind:data={prompts.current}
      oncreate={() => promptCreator.showModal()}
      moreActions={() => console.log('moreActions')}
    >
      {#snippet row(item)}
        <Markdown class="h-5" />
        <span class="list-col-grow text-base">{item.id}</span>
        <Button
          size="sm"
          icon={PencilSimpleLine}
          onclick={(event) => {
            event.stopPropagation();
            promptUpdater.showModal(item.id);
          }}
        />
      {/snippet}
    </List>
  </Setting>
  <Setting icon={SlidersHorizontal} title="常规设置">
    <div class="flex flex-col gap-1 px-1">
      <fieldset class="flex items-center justify-between">
        <Label icon={Translate}>语言设置</Label>
        <Select options={[{ value: 'zh-CN', label: '简体中文' }]} class="w-36 select-sm" />
      </fieldset>
      <div class="divider my-0 opacity-60"></div>
      <fieldset class="flex items-center justify-between">
        <Label icon={Swatches}>主题设置</Label>
        <Select
          options={[
            { value: 'light', label: '浅色' },
            { value: 'dracula', label: '深色' }
          ]}
          bind:value={theme.current}
          class="w-36 select-sm"
        />
      </fieldset>
      <div class="divider my-0 opacity-60"></div>
      <fieldset class="flex items-center justify-between">
        <Label icon={ClockCounterClockwise}>历史记录</Label>
        <Select
          options={[
            { value: 0, label: '不保留' },
            { value: 3, label: '最近 3 条' },
            { value: 5, label: '最近 5 条' },
            { value: 10, label: '最近 10 条' }
          ]}
          value={5}
          class="w-36 select-sm"
        />
      </fieldset>
    </div>
  </Setting>
</div>

<Model bind:this={modelCreator} models={models.current} />
<Model bind:this={modelUpdater} models={models.current} />

<Script bind:this={scriptCreator} scripts={scripts.current} />
<Script bind:this={scriptUpdater} scripts={scripts.current} />

<Prompt bind:this={promptCreator} prompts={prompts.current} />
<Prompt bind:this={promptUpdater} prompts={prompts.current} />
