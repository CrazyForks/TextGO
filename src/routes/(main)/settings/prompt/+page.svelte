<script lang="ts">
  import { Button, Label, List, Modal, Prompt, Setting } from '$lib/components';
  import { buildFormSchema } from '$lib/constraint';
  import { LMStudio, Ollama } from '$lib/icons';
  import { m } from '$lib/paraglide/messages';
  import { ollamaHost, prompts } from '$lib/stores.svelte';
  import { Cube, PencilSimpleLine, Robot, SlidersHorizontal, Sparkle } from 'phosphor-svelte';

  // form constraints
  const schema = buildFormSchema(({ text }) => ({
    ollamaHost: text().maxlength(256)
  }));

  // prompt
  let promptCreator: Prompt;
  let promptUpdater: Prompt;
  let promptOptions: Modal;
</script>

<Setting icon={Robot} title={m.ai_conversation()} moreOptions={() => promptOptions.show()} class="min-h-(--app-h)">
  <List
    icon={Sparkle}
    title={m.prompts_count({ count: prompts.current.length })}
    name={m.prompt_template()}
    hint={m.ai_conversation_hint()}
    bind:data={prompts.current}
    oncreate={() => promptCreator.showModal()}
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

<Prompt bind:this={promptCreator} prompts={prompts.current} />
<Prompt bind:this={promptUpdater} prompts={prompts.current} />

<Modal icon={SlidersHorizontal} title={m.ai_options()} bind:this={promptOptions}>
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
