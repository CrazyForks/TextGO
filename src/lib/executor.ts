import { PROMPT_MARK, SCRIPT_MARK } from '$lib/constants';
import { m } from '$lib/paraglide/messages';
import { entries, historySize, nodePath, prompts, pythonPath, scripts } from '$lib/stores.svelte';
import type { Entry, Option, Prompt, Rule, Script } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { readText, writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openPath, openUrl } from '@tauri-apps/plugin-opener';
import { memoize } from 'es-toolkit/function';
import {
  camelCase,
  capitalize,
  deburr,
  escape,
  kebabCase,
  lowerCase,
  pascalCase,
  reverseString,
  snakeCase,
  trim,
  trimEnd,
  trimStart,
  unescape,
  upperCase,
  words
} from 'es-toolkit/string';
import { Browsers, FolderOpen, Function, TextAa } from 'phosphor-svelte';

const URL_REGEX =
  /https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9()@:%_+.~#?&/=]*/gm;
const PATH_REGEX =
  /(?:[a-zA-Z]:\\[^<>:"|?*\n\r/]+(?:\\[^<>:"|?*\n\r/]+)*|~?\/[^<>:"|?*\n\r\\]+(?:\/[^<>:"|?*\n\r\\]+)*)/gm;

/**
 * 数据类型
 */
type Data = {
  /** 选中的文本 */
  selection: string;
  /** 剪贴板文本 */
  clipboard: string;
  /** 当前日期时间 */
  datetime: string;
};

/**
 * 内置动作类型
 */
type Processor = Option & {
  process: (selection: string) => string;
};

/**
 * 常规动作选项
 */
export const GENERAL_ACTIONS: Processor[] = [
  {
    value: 'open_urls',
    label: m.open_urls(),
    icon: Browsers,
    process: (text: string) => {
      // 提取文本中的所有 URL
      const urls = text.match(URL_REGEX) || [];
      // 打开每个 URL
      urls.forEach((url) => {
        openUrl(url).catch((error) => {
          console.error(`打开 ${url} 失败:`, error);
        });
      });
      return '';
    }
  },
  {
    value: 'open_files',
    label: m.open_files(),
    icon: FolderOpen,
    process: (text: string) => {
      // 提取文本中的所有文件路径
      const paths = text.match(PATH_REGEX) || [];
      // 打开每个文件路径
      paths.forEach((path) => {
        openPath(path).catch((error) => {
          console.error(`打开 ${path} 失败:`, error);
        });
      });
      return '';
    }
  }
];

/**
 * 格式转换动作选项
 */
export const CONVERT_ACTIONS: Processor[] = [
  {
    value: 'camel_case',
    label: m.camel_case(),
    icon: TextAa,
    process: camelCase
  },
  {
    value: 'pascal_case',
    label: m.pascal_case(),
    icon: TextAa,
    process: pascalCase
  },
  {
    value: 'snake_case',
    label: m.snake_case(),
    icon: TextAa,
    process: snakeCase
  },
  {
    value: 'kebab_case',
    label: m.kebab_case(),
    icon: TextAa,
    process: kebabCase
  },
  {
    value: 'lower_case',
    label: m.lower_case(),
    icon: TextAa,
    process: lowerCase
  },
  {
    value: 'upper_case',
    label: m.upper_case(),
    icon: TextAa,
    process: upperCase
  }
];

/**
 * 文本处理动作选项
 */
export const PROCESS_ACTIONS: Processor[] = [
  {
    value: 'lowercase',
    label: m.lowercase(),
    icon: Function,
    process: (text: string) => text.toLowerCase()
  },
  {
    value: 'uppercase',
    label: m.uppercase(),
    icon: Function,
    process: (text: string) => text.toUpperCase()
  },
  {
    value: 'capitalize',
    label: m.capitalize(),
    icon: Function,
    process: capitalize
  },
  {
    value: 'trim',
    label: m.trim(),
    icon: Function,
    process: trim
  },
  {
    value: 'ltrim',
    label: m.ltrim(),
    icon: Function,
    process: trimStart
  },
  {
    value: 'rtrim',
    label: m.rtrim(),
    icon: Function,
    process: trimEnd
  },
  {
    value: 'deburr',
    label: m.deburr(),
    icon: Function,
    process: deburr
  },
  {
    value: 'escape',
    label: m.escape(),
    icon: Function,
    process: escape
  },
  {
    value: 'unescape',
    label: m.unescape(),
    icon: Function,
    process: unescape
  },
  {
    value: 'reverse',
    label: m.reverse(),
    icon: Function,
    process: reverseString
  },
  {
    value: 'words',
    label: m.words(),
    icon: Function,
    process: (text: string) => words(text).join(' ')
  }
];

// Memoized 查找函数
const findBuiltinAction = memoize((action: string) =>
  [...GENERAL_ACTIONS, ...CONVERT_ACTIONS, ...PROCESS_ACTIONS].find((a) => a.value === action)
);

/**
 * 执行动作
 *
 * @param rule - 规则对象
 * @param selection - 选中的文本
 */
export async function execute(rule: Rule, selection: string): Promise<void> {
  // 动作标识
  const action = rule.action;
  // 组装数据
  const data: Data = {
    selection: selection,
    clipboard: await readText(),
    datetime: new Date().toISOString()
  };
  // 生成记录
  const entry: Entry = {
    id: crypto.randomUUID(),
    key: rule.key,
    caseLabel: rule.caseLabel,
    // actionLabel: rule.actionLabel,
    datetime: data.datetime,
    clipboard: data.clipboard,
    selection: data.selection
  };
  // 根据动作标识执行对应的操作
  if (action.startsWith(SCRIPT_MARK)) {
    const scriptId = action.substring(SCRIPT_MARK.length);
    const script = scripts.current.find((s) => s.id === scriptId);
    if (script) {
      console.debug('开始执行脚本:', scriptId);
      const result = await executeScript(script, data);
      console.debug('脚本执行成功:', result);
      // 保存记录
      entry.actionType = 'script';
      entry.actionLabel = scriptId;
      entry.result = result;
      entry.scriptLang = script.lang;
      entry.quietMode = script.quietMode;
      entries.current.unshift(entry);
      // 删除多余记录
      if (entries.current.length > historySize.current) {
        entries.current = entries.current.slice(0, historySize.current);
      }
      if (script.quietMode) {
        // 静默模式下不显示窗口
        await writeText(result);
        await invoke('send_paste_key');
      } else {
        await showPopup(entry);
      }
    }
  } else if (action.startsWith(PROMPT_MARK)) {
    const promptId = action.substring(PROMPT_MARK.length);
    const prompt = prompts.current.find((p) => p.id === promptId);
    if (prompt) {
      console.debug('开始生成提示词:', promptId);
      const result = await renderPrompt(prompt, data);
      console.debug('提示词生成成功:', result);
      // 保存记录
      entry.actionType = 'prompt';
      entry.actionLabel = promptId;
      entry.result = result;
      entry.systemPrompt = prompt.systemPrompt;
      entry.provider = prompt.provider;
      entry.model = prompt.model;
      entries.current.unshift(entry);
      // 删除多余记录
      if (entries.current.length > historySize.current) {
        entries.current = entries.current.slice(0, historySize.current);
      }
      await showPopup(entry);
    }
  } else {
    const builtin = findBuiltinAction(action);
    if (builtin) {
      console.debug('开始执行内置动作:', action);
      const result = builtin.process(selection);
      console.debug('内置动作执行成功:', result);
      await writeText(result);
      await invoke('send_paste_key');
    }
  }
}

/**
 * 执行输入的脚本并返回结果
 *
 * @param script - 脚本对象
 * @param data - 数据对象
 * @returns 脚本执行结果
 */
export async function executeScript(script: Script, data: Data): Promise<string> {
  try {
    if (script.lang === 'javascript') {
      const result = await invoke<string>('execute_javascript', {
        code: script.script,
        data: JSON.stringify(data),
        nodePath: nodePath.current
      });
      return result;
    } else if (script.lang === 'python') {
      const result = await invoke<string>('execute_python', {
        code: script.script,
        data: JSON.stringify(data),
        pythonPath: pythonPath.current
      });
      return result;
    } else {
      throw new Error(`不支持的脚本语言: ${script.lang}`);
    }
  } catch (error) {
    console.error('脚本执行失败:', error);
    throw error;
  }
}

/**
 * 渲染输入的提示词并返回结果
 *
 * @param prompt - 提示词对象
 * @param data - 数据对象
 * @returns 渲染结果
 */
export async function renderPrompt(prompt: Prompt, data: Data): Promise<string> {
  let result = prompt.prompt || '';

  // 使用正则表达式替换模板参数
  result = result.replace(/\{\{clipboard\}\}/g, data.clipboard);
  result = result.replace(/\{\{selection\}\}/g, data.selection);
  result = result.replace(/\{\{datetime\}\}/g, data.datetime);

  return result;
}

/**
 * 显示弹出窗口
 *
 * @param entry - 记录对象
 */
async function showPopup(entry: Entry): Promise<void> {
  try {
    await invoke('show_popup', { payload: JSON.stringify(entry) });
  } catch (error) {
    console.error('显示弹出窗口失败:', error);
  }
}
