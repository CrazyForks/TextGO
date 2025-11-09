<script lang="ts">
  import { Classifier } from '$lib/classifier';
  import {
    Button,
    Label,
    List,
    Modal,
    Model,
    Prompt,
    Regexp,
    Script as ScriptModal,
    Select,
    Setting
  } from '$lib/components';
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
  import { invoke } from '@tauri-apps/api/core';
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

  // current language
  let locale: Locale = $state(getLocale());

  // update tray menu language
  async function updateTrayMenu() {
    try {
      await invoke('setup_tray', {
        mainWindowText: m.tray_main_window(),
        shortcutsText: m.tray_shortcuts(),
        aboutText: m.tray_about(),
        quitText: m.tray_quit()
      });
    } catch (error) {
      console.error(`Failed to update tray menu language: ${error}`);
    }
  }

  // form constraints
  const schema = buildFormSchema(({ text }) => ({
    nodePath: text().maxlength(256),
    pythonPath: text().maxlength(256),
    ollamaHost: text().maxlength(256)
  }));

  // classification model
  let modelCreator: Model;
  let modelUpdater: Model;

  // regular expression
  let regexpCreator: Regexp;
  let regexpUpdater: Regexp;

  // script
  let scriptCreator: ScriptModal;
  let scriptUpdater: ScriptModal;
  let scriptOptions: Modal;

  // prompt
  let promptCreator: Prompt;
  let promptUpdater: Prompt;
  let promptOptions: Modal;
</script>

<div class="flex flex-col gap-2">
  <Setting icon={FingerprintSimple} title={m.text_type()} tip={m.text_type_tip()}>
    <List
      icon={Sphere}
      title={m.classification_model()}
      name={m.classification_model()}
      hint={m.classification_model_hint()}
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
              {vocabulary}
              {m.vocabulary()}
            </span>
          {/if}
        </div>
        {#if item.modelTrained === undefined}
          <div class="flex h-8 items-center gap-2 opacity-50">
            <span class="loading loading-sm loading-spinner"></span>
            {m.training()}
          </div>
        {:else if item.modelTrained === false}
          <div class="flex h-8 items-center gap-2 opacity-50">
            <Warning class="size-4 shrink-0" />
            {m.training_failed()}
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
      title={m.regexp()}
      name={m.regexp()}
      hint={m.regexp_hint()}
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
  <Setting icon={ArrowFatLineRight} title={m.trigger_action()} tip={m.trigger_action_tip()}>
    <List
      icon={Code}
      title={m.execute_script()}
      name={m.script()}
      hint={m.execute_script_hint()}
      bind:data={scripts.current}
      oncreate={() => scriptCreator.showModal()}
      moreOptions={() => scriptOptions.show()}
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
              {m.quiet_mode()}
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
      title={m.start_conversation()}
      name={m.prompt_template()}
      hint={m.start_conversation_hint()}
      bind:data={prompts.current}
      oncreate={() => promptCreator.showModal()}
      moreOptions={() => promptOptions.show()}
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
  <Setting icon={SlidersHorizontal} title={m.general_settings()}>
    <div class="flex flex-col gap-1 px-1">
      <fieldset class="flex items-center justify-between">
        <Label icon={Swatches}>{m.theme_settings()}</Label>
        <Select
          options={[
            { value: 'light', label: m.light_theme() },
            { value: 'dracula', label: m.dark_theme() }
          ]}
          bind:value={theme.current}
          class="w-36 select-sm"
        />
      </fieldset>
      <div class="divider my-0 opacity-60"></div>
      <fieldset class="flex items-center justify-between">
        <Label icon={Translate}>{m.language_settings()}</Label>
        <Select
          value={locale}
          options={[
            { value: 'en', label: 'English' },
            { value: 'zh-cn', label: '简体中文' }
          ]}
          class="w-36 select-sm"
          onchange={async (event) => {
            const target = event.currentTarget;
            locale = target.value as Locale;
            setLocale(locale);
            await updateTrayMenu();
          }}
        />
      </fieldset>
      <div class="divider my-0 opacity-60"></div>
      <fieldset class="flex items-center justify-between">
        <Label icon={ClockCounterClockwise}>{m.history_records()}</Label>
        <Select
          options={[
            { value: 0, label: m.history_none() },
            { value: 3, label: m.history_recent_3() },
            { value: 5, label: m.history_recent_5() },
            { value: 10, label: m.history_recent_10() },
            { value: 20, label: m.history_recent_20() }
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

<ScriptModal bind:this={scriptCreator} scripts={scripts.current} />
<ScriptModal bind:this={scriptUpdater} scripts={scripts.current} />

<Prompt bind:this={promptCreator} prompts={prompts.current} />
<Prompt bind:this={promptUpdater} prompts={prompts.current} />

<Modal icon={GearSix} title={m.script_options()} bind:this={scriptOptions}>
  <form>
    <fieldset class="fieldset">
      <Label>{m.nodejs_path()}</Label>
      <input
        class="input w-full"
        placeholder={m.nodejs_path_placeholder()}
        {...schema.nodePath}
        bind:value={nodePath.current}
      />
      <Label>{m.python_path()}</Label>
      <input
        class="input w-full"
        placeholder={m.python_path_placeholder()}
        {...schema.pythonPath}
        bind:value={pythonPath.current}
      />
    </fieldset>
  </form>
</Modal>

<Modal icon={GearSix} title={m.ai_options()} bind:this={promptOptions}>
  <form>
    <fieldset class="fieldset">
      <Label>{m.ollama_host()}</Label>
      <input
        class="input w-full"
        placeholder="http://127.0.0.1:11434"
        {...schema.ollamaHost}
        bind:value={ollamaHost.current}
      />
    </fieldset>
  </form>
</Modal>
