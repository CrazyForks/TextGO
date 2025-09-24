<script lang="ts">
  import { Alert, Confirm } from '$lib/components';
  import type { Snippet } from 'svelte';
  // 导入字体
  import '@fontsource-variable/noto-sans';
  import '@fontsource-variable/noto-sans-sc';
  // 导入样式
  import 'tippy.js/animations/scale.css';
  import 'tippy.js/dist/tippy.css';
  import '../app.css';

  let { children }: { children: Snippet } = $props();
</script>

{@render children()}

<!-- 全局提示组件 -->
<Alert />

<!-- 全局确认组件 -->
<Confirm />
