<script lang="ts" module>
  import type { Language, LanguageSupport } from '@codemirror/language';
  import type { Extension } from '@codemirror/state';
  import type { Options } from 'prettier';
  import babel from 'prettier/plugins/babel';
  import estree from 'prettier/plugins/estree';
  import html from 'prettier/plugins/html';
  import markdown from 'prettier/plugins/markdown';
  import postcss from 'prettier/plugins/postcss';
  import yaml from 'prettier/plugins/yaml';
  import * as prettier from 'prettier/standalone';

  export type CodeMirrorProps = Partial<{
    /** 文档语言 */
    language: LanguageSupport | Language | null;
    /** 文档内容 */
    document: string;
    /** 占位符文本 */
    placeholder: string;
    /** 制表符空格数 */
    tabSize: number;
    /** 行的最大长度 */
    lineLength: number;
    /** 编辑器是否为只读 */
    readOnly: boolean;
    /** 是否使用深色主题 */
    darkMode: boolean | 'auto';

    /** 容器的样式 */
    style: string | null;
    /** 容器的类名 */
    class: string;
    /** 面板的类名 */
    panelClass: string;
    /** 编辑器的类名 */
    editorClass: string;
    /** 编辑器的最小高度 */
    minHeight: string | null;
    /** 编辑器的最大高度 */
    maxHeight: string | null;
    /** 编辑器的最小宽度 */
    minWidth: string | null;
    /** 编辑器的最大宽度 */
    maxWidth: string | null;

    /** 大视图的标题 */
    title: string;
    /** 是否启用大视图功能 */
    enlarger: boolean;
    /** 是否在面板中显示复制按钮 */
    copier: boolean;
    /** 是否在面板中显示重置按钮 */
    resetter: boolean;
    /** 是否在面板中显示格式化按钮 */
    formatter: boolean;
    /** 文档更改时的回调函数 */
    onchange: (doc: string) => void;
  }>;

  /**
   * 从语言对象获取语言名称
   *
   * @param language - 语言对象
   */
  function getName(language: LanguageSupport | Language | null | undefined): string {
    if (!language) {
      // 默认为纯文本
      return 'text';
    }
    return 'name' in language ? language.name : language.language.name;
  }

  /**
   * 替换编辑器视图中的文档
   *
   * @param view - 编辑器视图
   * @param newDoc - 新文档
   */
  function replace(view: EditorView, newDoc: string | undefined) {
    // https://codemirror.net/examples/change/
    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: newDoc }
    });
  }

  /**
   * 根据语言类型格式化文档
   *
   * @param view - 编辑器视图
   * @param language - 文档语言
   * @param tabSize - 制表符空格数
   * @param lineLength - 行的最大长度
   */
  async function format(view: EditorView, language: string, tabSize: number, lineLength: number) {
    const formatOptions: Record<string, Options> = {
      javascript: { parser: 'babel', plugins: [babel, estree] },
      json: { parser: 'json', plugins: [babel, estree] },
      css: { parser: 'css', plugins: [postcss] },
      html: { parser: 'html', plugins: [html] },
      yaml: { parser: 'yaml', plugins: [yaml], singleQuote: true },
      markdown: { parser: 'markdown', plugins: [markdown] }
    };
    const option = formatOptions[language as keyof typeof formatOptions];
    if (!option) {
      return;
    }
    try {
      const source = view.state.doc.toString();
      const formatted = await prettier.format(source, {
        ...option,
        tabWidth: tabSize,
        printWidth: lineLength
      });
      replace(view, formatted);
    } catch (error) {
      console.error(error);
    }
  }

  /**
   * 中文翻译配置
   */
  const chinesePhrases: Record<string, string> = {
    // 搜索相关翻译
    Find: '查找',
    Replace: '替换',
    next: '下一个',
    previous: '上一个',
    all: '全部',
    'match case': '区分大小写',
    regexp: '正则表达式',
    'by word': '全字匹配',
    replace: '替换',
    'replace all': '替换全部',
    close: '关闭',
    'current match': '当前匹配',
    'on line': '在第',
    'replaced match on line $': '已替换第 $ 行的匹配项',
    'replaced $ matches': '已替换 $ 个匹配项',
    // 跳转相关翻译
    'Go to line': '跳转到行',
    go: '跳转'
  };
</script>

<script lang="ts">
  import { Button, Modal } from '$lib/components';
  import { theme } from '$lib/states.svelte';
  import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from '@codemirror/autocomplete';
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
  import {
    bracketMatching,
    defaultHighlightStyle,
    foldGutter,
    foldKeymap,
    indentOnInput,
    indentUnit,
    syntaxHighlighting
  } from '@codemirror/language';
  import { lintKeymap } from '@codemirror/lint';
  import { highlightSelectionMatches, searchKeymap } from '@codemirror/search';
  import { Compartment, EditorState } from '@codemirror/state';
  import { oneDark } from '@codemirror/theme-one-dark';
  import {
    crosshairCursor,
    drawSelection,
    dropCursor,
    EditorView,
    highlightActiveLine,
    highlightActiveLineGutter,
    highlightSpecialChars,
    keymap,
    lineNumbers,
    placeholder,
    rectangularSelection
  } from '@codemirror/view';
  import { ArrowCounterClockwise, Code, CopySimple, FrameCorners, TextIndent } from 'phosphor-svelte';
  import { onMount } from 'svelte';
  import CodeMirror from './CodeMirror.svelte';

  let {
    language,
    document = $bindable(),
    placeholder: _placeholder,
    tabSize = 4,
    lineLength = 80,
    readOnly = false,
    darkMode = 'auto',

    style,
    class: _class,
    panelClass,
    editorClass,
    minHeight = '8rem',
    maxHeight = '20rem',
    minWidth = '100%',
    maxWidth = '100%',

    title = '',
    enlarger = !readOnly,
    copier = true,
    resetter = true,
    formatter = true,
    onchange
  }: CodeMirrorProps = $props();

  let editor: HTMLDivElement;
  let editorView: EditorView;
  let largerView: Modal | null = $state(null);
  let originalDoc = document;
  const languageName = getName(language);

  /**
   * 用新文档替换当前文档
   *
   * @param newDoc - 新文档
   * @param setOriginal - 是否替换原始文档
   */
  export function setDocument(newDoc: string | undefined, setOriginal: boolean = false) {
    replace(editorView, newDoc);
    if (setOriginal) {
      originalDoc = newDoc;
    }
  }

  /**
   * 编辑器的基本扩展集合
   *
   * https://github.com/codemirror/basic-setup
   */
  const basicSetup: Extension = [
    lineNumbers(),
    indentOnInput(),
    history(),
    autocompletion(),
    closeBrackets(),
    bracketMatching(),
    dropCursor(),
    crosshairCursor(),
    drawSelection(),
    rectangularSelection(),
    EditorState.phrases.of(chinesePhrases),
    EditorState.allowMultipleSelections.of(true),
    syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
    highlightSpecialChars(),
    // foldGutter(),
    // highlightActiveLine(),
    // highlightActiveLineGutter(),
    // highlightSelectionMatches(),
    keymap.of([
      ...defaultKeymap,
      ...foldKeymap,
      ...historyKeymap,
      ...completionKeymap,
      ...closeBracketsKeymap,
      ...searchKeymap,
      ...lintKeymap
    ])
  ];

  /**
   * 为编辑器添加样式的扩展
   *
   * https://codemirror.net/examples/styling/
   */
  const styleSheets: Extension = [
    EditorView.baseTheme({
      '&': {
        userSelect: 'text'
      },
      '.cm-content, .cm-gutter': {
        cursor: 'text'
      },
      '.cm-scroller': {
        overflow: 'auto',
        overscrollBehavior: 'none'
      },
      '&.cm-focused': {
        outline: 'none'
      },
      '.cm-gutters': {
        backgroundColor: 'var(--color-base-200)',
        border: 'none'
      }
    }),
    EditorView.theme({
      '&': {
        minWidth: minWidth,
        maxWidth: maxWidth,
        maxHeight: maxHeight
      },
      '.cm-content, .cm-gutter': {
        minHeight: minHeight
      }
    })
  ];

  /**
   * 支持指定语言的扩展
   */
  const languageSupport: Extension = language ? language : [];

  /**
   * 监听文档更改的扩展
   */
  const updateListener: Extension = EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      document = update.state.doc.toString();
      onchange?.(document);
    }
  });

  /**
   * 处理制表符键的扩展
   *
   * https://codemirror.net/examples/tab/
   */
  const tabKeyHandler: Extension = [keymap.of([indentWithTab]), indentUnit.of(' '.repeat(tabSize))];

  /**
   * 编辑器主题的扩展
   */
  const getTheme = () => (darkMode === true || (darkMode === 'auto' && theme.current !== 'light') ? oneDark : []);
  const editorTheme = new Compartment();
  const themeHandler: Extension = editorTheme.of(getTheme());
  $effect(() => {
    if (theme.current && editorView) {
      editorView.dispatch({
        effects: editorTheme.reconfigure(getTheme())
      });
    }
  });

  /**
   * 使编辑器只读的扩展
   */
  const readOnlyHandler: Extension = (() => {
    if (readOnly) {
      return [EditorState.readOnly.of(true), EditorView.editable.of(false)];
    }
    // 只有当编辑器可编辑时才高亮活动行
    return [foldGutter(), highlightActiveLine(), highlightActiveLineGutter(), highlightSelectionMatches()];
  })();

  /**
   * 启用占位符文本的扩展
   */
  const placeholderHandler: Extension = _placeholder ? placeholder(_placeholder) : [];

  onMount(() => {
    // 创建编辑器视图
    editorView = new EditorView({
      parent: editor,
      // 创建编辑器状态
      state: EditorState.create({
        doc: document,
        extensions: [
          basicSetup,
          styleSheets,
          languageSupport,
          updateListener,
          tabKeyHandler,
          themeHandler,
          readOnlyHandler,
          placeholderHandler
        ]
      })
    });
    return () => {
      // 销毁编辑器视图
      editorView.destroy();
    };
  });
</script>

<svelte:window
  onkeydown={(event) => {
    if ((event.ctrlKey || event.metaKey) && event.code === 'KeyZ') {
      // 撤销/重做触发时更新文档
      setTimeout(() => setDocument(document), 0);
    }
  }}
/>

<div class="shrink-0 overflow-auto rounded-box border-1 border-base-content/20 {_class}" {style}>
  <div class={editorClass} bind:this={editor}></div>
  <div class="flex items-center justify-between border-t px-2 py-1 {panelClass}">
    <span class="flex items-center gap-2">
      {#if enlarger}
        <Button
          icon={FrameCorners}
          class="border-0 bg-transparent shadow-none {enlarger ? '' : 'pointer-events-none'}"
          onclick={() => enlarger && largerView?.show()}
        />
      {/if}
      <span class="text-xs font-semibold opacity-70">{languageName}</span>
    </span>
    <span class="flex items-center gap-2">
      {#if !readOnly && resetter}
        <Button
          icon={ArrowCounterClockwise}
          class="border-0 bg-transparent shadow-none"
          onclick={() => replace(editorView, originalDoc)}
        />
      {/if}
      {#if !readOnly && language && formatter}
        <Button
          icon={TextIndent}
          class="border-0 bg-transparent shadow-none"
          onclick={() => format(editorView, languageName, tabSize, lineLength)}
        />
      {/if}
      {#if copier}
        <Button
          icon={CopySimple}
          class="border-0 bg-transparent shadow-none"
          onclick={() => document && navigator.clipboard && navigator.clipboard.writeText(document)}
        />
      {/if}
    </span>
  </div>
</div>

{#if enlarger}
  <Modal icon={Code} title={title || languageName} maxWidth="80rem" bind:this={largerView}>
    <CodeMirror
      {language}
      bind:document
      placeholder={_placeholder}
      {tabSize}
      {lineLength}
      {readOnly}
      {darkMode}
      {style}
      class={_class}
      {panelClass}
      {editorClass}
      minHeight="60dvh"
      maxHeight="calc(90dvh - 10rem)"
      minWidth="100%"
      maxWidth="100%"
      enlarger={false}
      {copier}
      {resetter}
      {formatter}
      onchange={() => replace(editorView, document)}
    />
  </Modal>
{/if}
