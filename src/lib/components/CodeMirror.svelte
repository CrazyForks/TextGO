<script lang="ts" module>
  import { m } from '$lib/paraglide/messages';
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
   * 从语言支持对象中获取语言名称
   *
   * @param language - 语言支持对象
   */
  function getLanguageName(language: LanguageSupport | Language | null | undefined): string {
    let name = null;
    if (language) {
      name = 'name' in language ? language.name : language.language.name;
    } else {
      // 默认为纯文本
      name = 'text';
    }
    // 非首字母大写的语言名称映射
    const mappings: Record<string, string> = {
      javascript: 'JavaScript',
      json: 'JSON',
      css: 'CSS',
      html: 'HTML',
      yaml: 'YAML'
    };
    // 返回映射名称或首字母大写名称
    return mappings[name] || name.charAt(0).toUpperCase() + name.slice(1);
  }

  /**
   * 替换编辑器视图中的文档
   *
   * @param view - 编辑器视图
   * @param newDoc - 新文档
   */
  function replaceDocument(view: EditorView, newDoc: string | undefined) {
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
  async function formatDocument(view: EditorView, language: string, tabSize: number, lineLength: number) {
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
      replaceDocument(view, formatted);
    } catch (error) {
      console.error('格式化文档失败:', error);
    }
  }

  /**
   * 编辑器中短语的翻译
   */
  const phrases: Record<string, string> = {
    // 搜索相关翻译
    Find: m.codemirror_find(),
    Replace: m.codemirror_replace(),
    next: m.codemirror_next(),
    previous: m.codemirror_previous(),
    all: m.codemirror_all(),
    'match case': m.codemirror_match_case(),
    regexp: m.codemirror_regexp(),
    'by word': m.codemirror_by_word(),
    replace: m.codemirror_replace_action(),
    'replace all': m.codemirror_replace_all(),
    close: m.codemirror_close(),
    'current match': m.codemirror_current_match(),
    'on line': m.codemirror_on_line(),
    'replaced match on line $': m.codemirror_replaced_match_on_line(),
    'replaced $ matches': m.codemirror_replaced_matches(),
    // 跳转相关翻译
    'Go to line': m.codemirror_go_to_line(),
    go: m.codemirror_go()
  };
</script>

<script lang="ts">
  import { Button, Modal } from '$lib/components';
  import { theme } from '$lib/stores.svelte';
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
  const languageName = getLanguageName(language);

  /**
   * 重置文档内容
   */
  export function reset() {
    replaceDocument(editorView, originalDoc);
  }

  /**
   * 格式化文档内容
   */
  export function format() {
    formatDocument(editorView, languageName, tabSize, lineLength);
  }

  /**
   * 复制文档内容到剪贴板
   */
  export function copy() {
    document && navigator.clipboard && navigator.clipboard.writeText(document);
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
    EditorState.phrases.of(phrases),
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

<div class="shrink-0 overflow-auto rounded-box border-1 border-base-content/20 {_class}">
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
        <Button icon={ArrowCounterClockwise} class="border-0 bg-transparent shadow-none" onclick={reset} />
      {/if}
      {#if !readOnly && language && formatter}
        <Button icon={TextIndent} class="border-0 bg-transparent shadow-none" onclick={format} />
      {/if}
      {#if copier}
        <Button icon={CopySimple} class="border-0 bg-transparent shadow-none" onclick={copy} />
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
      minHeight="60dvh"
      maxHeight="calc(90dvh - 10rem)"
      minWidth="100%"
      maxWidth="100%"
      enlarger={false}
      {copier}
      {resetter}
      {formatter}
      onchange={() => replaceDocument(editorView, document)}
    />
  </Modal>
{/if}
