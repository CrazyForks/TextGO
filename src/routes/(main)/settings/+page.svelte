<script lang="ts">
  import { Classifier } from '$lib/classifier';
  import { Button, Label, List, Modal, Model, Prompt, Regexp, Script, Select, Setting } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { JavaScript, LMStudio, Ollama, Python, Regexp as RegexpIcon, Tensorflow } from '$lib/icons';
  import { m } from '$lib/paraglide/messages';
  import { getLocale, setLocale, type Locale } from '$lib/paraglide/runtime';
  import {
    historySize,
    models,
    nodePath,
    ollamaHost,
    prompts,
    pythonPath,
    regexps,
    scripts,
    theme
  } from '$lib/stores.svelte';
  import {
    ArrowFatLineRight,
    ClockCounterClockwise,
    Code,
    Cube,
    Empty,
    FingerprintSimple,
    GearSix,
    Package,
    PencilSimpleLine,
    Robot,
    Scroll,
    SlidersHorizontal,
    Sphere,
    Swatches,
    TextT,
    Translate,
    Warning
  } from 'phosphor-svelte';

  // 当前语言
  let locale: Locale = $state(getLocale());

  // 表单约束
  const schema = buildFormSchema(({ text }) => ({
    nodePath: text().maxlength(256),
    pythonPath: text().maxlength(256),
    ollamaHost: text().maxlength(256)
  }));

  // 分类模型
  let modelCreator: Model;
  let modelUpdater: Model;

  // 正则表达式
  let regexpCreator: Regexp;
  let regexpUpdater: Regexp;

  // 脚本
  let scriptCreator: Script;
  let scriptUpdater: Script;
  let scriptOptions: Modal;

  // 提示词
  let promptCreator: Prompt;
  let promptUpdater: Prompt;
  let promptOptions: Modal;
</script>

<div class="flex flex-col gap-2">
  <Setting icon={FingerprintSimple} title={m.text_type()} tip="自定义可识别的文本类型">
    <List
      icon={Sphere}
      title="分类模型"
      name="分类模型"
      hint="训练分类模型识别自定义类型"
      bind:data={models.current}
      oncreate={() => modelCreator.showModal()}
      ondelete={(item) => Classifier.clearSavedModel(item.id)}
    >
      {#snippet row(item)}
        <Tensorflow class="h-5" />
        <div class="list-col-grow flex items-center gap-4 truncate" title={item.id}>
          <span class="truncate text-base font-light">{item.id}</span>
          {#if item.modelTrained === true}
            {@const { sizeKB, vocabulary } = Classifier.getModelInfo(item.id)}
            <span class="badge badge-ghost badge-sm">
              <Package class="size-4 shrink-0 opacity-50" />
              {sizeKB} KB
            </span>
            <span class="badge badge-ghost badge-sm">
              <TextT class="size-4 shrink-0 opacity-50" />
              {vocabulary} 词汇
            </span>
          {/if}
        </div>
        {#if item.modelTrained === undefined}
          <div class="flex h-8 items-center gap-2 opacity-50">
            <span class="loading loading-sm loading-spinner"></span>
            训练中
          </div>
        {:else if item.modelTrained === false}
          <div class="flex h-8 items-center gap-2 opacity-50">
            <Warning class="size-4 shrink-0" />
            训练失败
          </div>
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
      hint="编写正则表达式匹配自定义类型"
      bind:data={regexps.current}
      oncreate={() => regexpCreator.showModal()}
    >
      {#snippet row(item)}
        <RegexpIcon class="h-5" />
        <div class="list-col-grow flex items-center gap-4 truncate" title={item.id}>
          <span class="truncate text-base font-light">{item.id}</span>
        </div>
        <Button
          size="sm"
          icon={PencilSimpleLine}
          onclick={(event) => {
            event.stopPropagation();
            regexpUpdater.showModal(item.id);
          }}
        />
      {/snippet}
    </List>
  </Setting>
  <Setting icon={ArrowFatLineRight} title="触发动作" tip="自定义识别文本后可执行的动作">
    <List
      icon={Code}
      title="执行脚本"
      name="脚本"
      hint="执行预设的脚本处理选中文本"
      bind:data={scripts.current}
      oncreate={() => scriptCreator.showModal()}
      moreActions={() => scriptOptions.show()}
    >
      {#snippet row(item)}
        {#if item.lang === 'javascript'}
          <JavaScript class="h-5" />
        {:else if item.lang === 'python'}
          <Python class="h-5" />
        {/if}
        <div class="list-col-grow flex items-center gap-4 truncate" title={item.id}>
          <span class="truncate text-base font-light">{item.id}</span>
          {#if item.quietMode === true}
            <span class="badge badge-ghost badge-sm">
              <Empty class="size-4 shrink-0 opacity-50" />
              静默模式
            </span>
          {/if}
        </div>
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
      icon={Robot}
      title="发起对话"
      name="提示词模板"
      hint="通过预设的提示词与 AI 模型对话"
      bind:data={prompts.current}
      oncreate={() => promptCreator.showModal()}
      moreActions={() => promptOptions.show()}
    >
      {#snippet row(item)}
        {#if item.provider === 'ollama'}
          <Ollama class="h-5" />
        {:else if item.provider === 'lmstudio'}
          <LMStudio class="h-5" />
        {/if}
        <div class="list-col-grow flex items-center gap-4 truncate" title={item.id}>
          <span class="truncate text-base font-light">{item.id}</span>
          <span class="badge badge-ghost badge-sm">
            <Cube class="size-4 shrink-0 opacity-50" />
            {item.model}
          </span>
        </div>
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
        <Select
          value={locale}
          options={[
            { value: 'en', label: 'English' },
            { value: 'zh-cn', label: '简体中文' }
          ]}
          class="w-36 select-sm"
          onchange={(event) => {
            const target = event.currentTarget;
            locale = target.value as Locale;
            setLocale(locale);
          }}
        />
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
            { value: 10, label: '最近 10 条' },
            { value: 20, label: '最近 20 条' }
          ]}
          bind:value={historySize.current}
          class="w-36 select-sm"
        />
      </fieldset>
    </div>
  </Setting>
</div>

<Model bind:this={modelCreator} models={models.current} />
<Model bind:this={modelUpdater} models={models.current} />

<Regexp bind:this={regexpCreator} regexps={regexps.current} />
<Regexp bind:this={regexpUpdater} regexps={regexps.current} />

<Script bind:this={scriptCreator} scripts={scripts.current} />
<Script bind:this={scriptUpdater} scripts={scripts.current} />

<Prompt bind:this={promptCreator} prompts={prompts.current} />
<Prompt bind:this={promptUpdater} prompts={prompts.current} />

<Modal icon={GearSix} title="脚本选项设置" bind:this={scriptOptions}>
  <form>
    <fieldset class="fieldset">
      <Label>Node.js 路径</Label>
      <input
        class="input w-full"
        placeholder="例如: /usr/local/bin/node"
        {...schema.nodePath}
        bind:value={nodePath.current}
      />
      <Label>Python 路径</Label>
      <input
        class="input w-full"
        placeholder="例如: /usr/local/bin/python3"
        {...schema.pythonPath}
        bind:value={pythonPath.current}
      />
    </fieldset>
  </form>
</Modal>

<Modal icon={GearSix} title="AI 选项设置" bind:this={promptOptions}>
  <form>
    <fieldset class="fieldset">
      <Label>Ollama 服务地址</Label>
      <input
        class="input w-full"
        placeholder="http://127.0.0.1:11434"
        {...schema.ollamaHost}
        bind:value={ollamaHost.current}
      />
    </fieldset>
  </form>
</Modal>
