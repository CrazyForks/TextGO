<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  // 默认跳转至快捷键注册页
  goto(resolve('/shortcuts'), { replaceState: true });
</script>
