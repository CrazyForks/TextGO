<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { NoData } from '$lib/icons';
  import { entries } from '$lib/stores.svelte';

  if (entries.current.length > 0) {
    // if history exists, redirect to first history detail page
    goto(resolve(`/histories/${entries.current[0].id}`), { replaceState: true });
  }
</script>

<div class="pointer-events-none absolute inset-0 flex items-center justify-center">
  <NoData class="m-auto size-64 pl-4 opacity-10" />
</div>
