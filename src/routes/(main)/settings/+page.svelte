<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  goto(resolve('/settings/general'), { replaceState: true });
</script>
