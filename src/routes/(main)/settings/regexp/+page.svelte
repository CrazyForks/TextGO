<script lang="ts">
  import { Button, List, Regexp, Setting } from '$lib/components';
  import { Regexp as RegexpIcon } from '$lib/icons';
  import { m } from '$lib/paraglide/messages';
  import { regexps } from '$lib/stores.svelte';
  import { PencilSimpleLine, Scroll, Sparkle } from 'phosphor-svelte';

  // regular expression
  let regexpCreator: Regexp;
  let regexpUpdater: Regexp;
</script>

<Setting icon={Scroll} title={m.regexp()} class="min-h-(--app-h)">
  <List
    icon={Sparkle}
    title={m.regexps_count({ count: regexps.current.length })}
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

<Regexp bind:this={regexpCreator} regexps={regexps.current} />
<Regexp bind:this={regexpUpdater} regexps={regexps.current} />
