<script lang="ts">
  import { goto } from '$app/navigation';

  // 默认跳转至快捷键注册页
  goto('/shortcuts', { replaceState: true });
</script>
