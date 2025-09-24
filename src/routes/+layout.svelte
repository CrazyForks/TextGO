<script lang="ts">
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
