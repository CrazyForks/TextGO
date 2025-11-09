<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  // default redirect to shortcut key registration page
  goto(resolve('/shortcuts'), { replaceState: true });
</script>
