import { getClipboardText, setClipboardText } from '$lib/clipboard';
import { PROMPT_MARK, PROMPTS_KEY, SCRIPT_MARK, SCRIPTS_KEY } from '$lib/constants';
import { entries, getPersisted } from '$lib/states.svelte';
import type { Entry, Hotkey, Prompt, Script } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * 数据类型
 */
type Data = {
  /** 选中的文本 */
  selection: string;
  /** 剪贴版文本 */
  clipboard: string;
  /** 当前日期时间 */
  datetime: string;
};

/**
 * 执行动作
 *
 * @param hotkey - 快捷键对象
 * @param selection - 选中的文本
 */
export async function execute(hotkey: Hotkey, selection: string): Promise<void> {
  // 动作标识
  const action = hotkey.action;
  // 组装数据
  const data: Data = {
    selection: selection,
    clipboard: await getClipboardText(),
    datetime: new Date().toISOString()
  };
  // 生成记录
  const entry: Entry = {
    id: crypto.randomUUID(),
    key: hotkey.key,
    caseLabel: hotkey.caseLabel,
    // actionLabel: hotkey.actionLabel,
    datetime: data.datetime,
    clipboard: data.clipboard,
    selection: data.selection
  };
  // 根据动作标识执行对应的操作
  if (action.startsWith(SCRIPT_MARK)) {
    const scriptId = action.substring(SCRIPT_MARK.length);
    const scripts = await getPersisted<Script[]>(SCRIPTS_KEY);
    const script = scripts?.find((s) => s.id === scriptId);
    if (script) {
      console.debug(`开始执行脚本: ${scriptId}`);
      const result = await executeScript(script, data);
      console.debug(`脚本执行成功: ${result}`);
      if (script.quietMode) {
        // 静默模式下不保存记录和显示窗口
        await setClipboardText(result);
        await invoke('send_paste_key');
        return;
      }
      // 保存记录
      entry.actionType = 'script';
      entry.scriptLang = script.lang;
      entry.actionLabel = scriptId;
      entry.result = result;
      entries.current.unshift(entry);
      // 保留最近5条记录
      if (entries.current.length > 5) {
        entries.current = entries.current.slice(0, 5);
      }
      await showWindow(entry);
    }
  } else if (action.startsWith(PROMPT_MARK)) {
    const promptId = action.substring(PROMPT_MARK.length);
    const prompts = await getPersisted<Prompt[]>(PROMPTS_KEY);
    const prompt = prompts?.find((p) => p.id === promptId);
    if (prompt) {
      console.debug(`开始生成提示词: ${promptId}`);
      const result = await renderPrompt(prompt, data);
      console.debug(`提示词生成成功: ${result}`);
      // 保存记录
      entry.actionType = 'prompt';
      entry.actionLabel = promptId;
      entry.result = result;
      entry.systemPrompt = prompt.systemPrompt;
      entry.provider = prompt.provider;
      entry.model = prompt.model;
      entries.current.unshift(entry);
      // 保留最近5条记录
      if (entries.current.length > 5) {
        entries.current = entries.current.slice(0, 5);
      }
      await showWindow(entry);
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
        data: JSON.stringify(data)
      });
      return result;
    } else if (script.lang === 'python') {
      const result = await invoke<string>('execute_python', {
        code: script.script,
        data: JSON.stringify(data)
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
  result = result.replace(/\{\{selection\}\}/g, data.selection);
  result = result.replace(/\{\{clipboard\}\}/g, data.clipboard);
  result = result.replace(/\{\{datetime\}\}/g, data.datetime);

  return result;
}

/**
 * 显示窗口
 */
async function showWindow(entry: Entry): Promise<void> {
  try {
    await invoke('show_popup_window', { payload: JSON.stringify(entry) });
  } catch (error) {
    console.error('显示窗口失败:', error);
  }
}
