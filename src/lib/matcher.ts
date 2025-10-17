import { predict } from '$lib/classifier';
import { MODEL_MARK } from '$lib/constants';
import { models } from '$lib/stores.svelte';
import type { Hotkey, Model, Option } from '$lib/types';
import { ModelOperations, type ModelResult } from '@vscode/vscode-languagedetection';
import { franc } from 'franc-min';

// 创建编程语言识别模型实例
// 针对 Tauri 环境配置自定义加载函数
const modelOperations = new ModelOperations({
  // 自定义 JSON 模型加载函数
  modelJsonLoaderFunc: async () => {
    try {
      const response = await fetch('/model.json');
      if (!response.ok) {
        throw new Error(`无法加载模型 JSON 文件: ${response.status}`);
      }
      return await response.json();
    } catch (error) {
      console.error('加载模型 JSON 文件失败:', error);
      throw error;
    }
  },
  // 自定义权重文件加载函数
  weightsLoaderFunc: async () => {
    try {
      const response = await fetch('/group1-shard1of1.bin');
      if (!response.ok) {
        throw new Error(`无法加载模型权重文件: ${response.status}`);
      }
      return await response.arrayBuffer();
    } catch (error) {
      console.error('加载模型权重文件失败:', error);
      throw error;
    }
  }
});

/**
 * 最低期望置信度
 */
const MIN_CONFIDENCE = 0.2;

/**
 * 初始置信度阈值
 */
const INITIAL_THRESHOLD = 0.5;

/**
 * 相对置信度差值阈值
 */
const RELATIVE_THRESHOLD = 0.15;

/**
 * 内置正则识别选项
 */
export const BUILTIN_CASES: Option[] = [
  {
    value: 'url',
    label: '网址',
    pattern:
      /^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)$/
  },
  {
    value: 'email',
    label: '邮箱',
    pattern:
      /^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/
  },
  {
    value: 'ipv4',
    label: 'IPv4 地址',
    pattern:
      /^(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/
  },
  {
    value: 'ipv6',
    label: 'IPv6 地址',
    pattern:
      /^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$/
  }
];

/**
 * 自然语言识别选项
 */
export const NATURAL_CASES: Option[] = [
  { value: 'cmn', label: '中文' },
  { value: 'eng', label: '英文' },
  { value: 'jpn', label: '日文' },
  { value: 'kor', label: '韩文' },
  { value: 'rus', label: '俄文' },
  { value: 'fra', label: '法文' },
  { value: 'deu', label: '德文' },
  { value: 'spa', label: '西班牙文' },
  { value: 'por', label: '葡萄牙文' },
  { value: 'arb', label: '阿拉伯文' }
];

/**
 * 编程语言识别选项
 */
export const PROGRAM_CASES: Option[] = [
  { value: 'asm', label: 'Assembly' },
  { value: 'bat', label: 'Batchfile' },
  { value: 'c', label: 'C' },
  { value: 'cs', label: 'C#' },
  { value: 'cpp', label: 'C++' },
  { value: 'clj', label: 'Clojure' },
  { value: 'cmake', label: 'CMake' },
  { value: 'cbl', label: 'COBOL' },
  { value: 'coffee', label: 'CoffeeScript' },
  { value: 'css', label: 'CSS' },
  { value: 'csv', label: 'CSV' },
  { value: 'dart', label: 'Dart' },
  { value: 'dm', label: 'DM' },
  { value: 'dockerfile', label: 'Dockerfile' },
  { value: 'ex', label: 'Elixir' },
  { value: 'erl', label: 'Erlang' },
  { value: 'f90', label: 'Fortran' },
  { value: 'go', label: 'Go' },
  { value: 'groovy', label: 'Groovy' },
  { value: 'hs', label: 'Haskell' },
  { value: 'html', label: 'HTML' },
  { value: 'ini', label: 'INI' },
  { value: 'java', label: 'Java' },
  { value: 'js', label: 'JavaScript' },
  { value: 'json', label: 'JSON' },
  { value: 'jl', label: 'Julia' },
  { value: 'kt', label: 'Kotlin' },
  { value: 'lisp', label: 'Lisp' },
  { value: 'lua', label: 'Lua' },
  { value: 'makefile', label: 'Makefile' },
  { value: 'md', label: 'Markdown' },
  { value: 'matlab', label: 'Matlab' },
  { value: 'mm', label: 'Objective-C' },
  { value: 'ml', label: 'OCaml' },
  { value: 'pas', label: 'Pascal' },
  { value: 'pm', label: 'Perl' },
  { value: 'php', label: 'PHP' },
  { value: 'ps1', label: 'PowerShell' },
  { value: 'prolog', label: 'Prolog' },
  { value: 'py', label: 'Python' },
  { value: 'r', label: 'R' },
  { value: 'rb', label: 'Ruby' },
  { value: 'rs', label: 'Rust' },
  { value: 'scala', label: 'Scala' },
  { value: 'sh', label: 'Shell' },
  { value: 'sql', label: 'SQL' },
  { value: 'swift', label: 'Swift' },
  { value: 'tex', label: 'TeX' },
  { value: 'toml', label: 'TOML' },
  { value: 'ts', label: 'TypeScript' },
  { value: 'v', label: 'Verilog' },
  { value: 'vba', label: 'Visual Basic' },
  { value: 'xml', label: 'XML' },
  { value: 'yaml', label: 'YAML' }
];

/**
 * 根据文本类型匹配要执行的快捷键动作
 *
 * @param text - 待匹配的文本
 * @param keys - 指定的快捷键标识列表
 * @returns 匹配到的快捷键对象，未匹配到则返回`null`
 */
export async function match(text: string, hotkeys: Hotkey[]): Promise<Hotkey | null> {
  console.debug(`获取待匹配模式: ${hotkeys.map((h) => h.case || '跳过').join('、')}`);
  let langDetected = false;
  let langDetectedResult: ModelResult[] = [];
  // 循环所有绑定的快捷键，找到第一个匹配的文本类型
  for (const hotkey of hotkeys) {
    const _case = hotkey.case;
    // 空字符串表示不进行任何识别
    if (_case === '') {
      console.debug('%c跳过文本识别', 'color: rgba(0,0,0,0.5)');
      return hotkey;
    }
    // 内置正则匹配
    const builtin = BUILTIN_CASES.find((c) => c.value === _case);
    if (builtin && builtin.pattern && builtin.pattern.test(text)) {
      console.debug(`内置正则表达式匹配成功: ${builtin.label}`);
      hotkey.caseLabel = builtin.label;
      return hotkey;
    }
    // 自然语言识别
    const natural = NATURAL_CASES.find((c) => c.value === _case);
    if (natural) {
      try {
        if (franc(text, { minLength: 2 }) === _case) {
          console.debug(`自然语言识别成功: ${natural.label}`);
          hotkey.caseLabel = natural.label;
          return hotkey;
        }
      } catch (error) {
        console.error('自然语言识别失败:', error);
      }
    }
    // 编程语言识别
    const program = PROGRAM_CASES.find((c) => c.value === _case);
    if (program) {
      try {
        if (!langDetected) {
          langDetectedResult = await modelOperations.runModel(text);
          langDetected = true;
          console.debug('编程语言识别结果:', langDetectedResult);
        }
        if (matchProgramCase(_case, langDetectedResult)) {
          console.debug(`编程语言识别成功: ${program.label}`);
          hotkey.caseLabel = program.label;
          return hotkey;
        }
      } catch (error) {
        console.error('编程语言识别失败:', error);
      }
    }
    // 自定义模型识别
    if (_case.startsWith(MODEL_MARK)) {
      const modelId = _case.substring(MODEL_MARK.length);
      const model = models.current.find((m) => m.id === modelId);
      if (model) {
        try {
          if (await matchModelCase(model, text)) {
            console.debug(`自定义模型识别成功: ${model.id}`);
            hotkey.caseLabel = model.id;
            return hotkey;
          }
        } catch (error) {
          console.error('自定义模型识别失败:', error);
        }
      }
    }
  }
  return null;
}

/**
 * 判断编程语言识别结果是否匹配目标语言
 * 参考了 VS Code 的识别策略，使用了动态阈值和相邻位置的置信度差值来判断
 * https://github.com/microsoft/vscode/blob/main/src/vs/workbench/services/languageDetection/browser/languageDetectionWebWorker.ts
 *
 * @param targetId - 目标语言ID
 * @param results - 模型识别结果
 * @returns 是否匹配目标语言
 */
function matchProgramCase(targetId: string, results: ModelResult[]): boolean {
  if (!results || results.length === 0) {
    return false;
  }
  // 获取目标语言在模型识别结果中的位置和置信度
  const targetIndex = results.findIndex((result) => result.languageId === targetId);
  if (targetIndex === -1) {
    // 目标语言不在结果中
    return false;
  }
  const targetConfidence = results[targetIndex].confidence;

  // 策略1: 判断目标语言置信度是否达标
  const threshold = INITIAL_THRESHOLD + 0.1 * targetIndex;
  if (targetConfidence > threshold) {
    return true;
  }

  // 策略2: 判断相邻置信度差值是否达标
  if (targetConfidence <= MIN_CONFIDENCE || targetIndex >= 3) {
    // 置信度过低或排名过后
    return false;
  }
  const nextConfidence = results[targetIndex + 1]?.confidence ?? 0;
  return targetConfidence - nextConfidence > RELATIVE_THRESHOLD;
}

/**
 * 判断选中文本是否匹配自定义模型
 *
 * @param model - 自定义模型
 * @param text - 待匹配文本
 * @returns
 */
async function matchModelCase(model: Model, text: string): Promise<boolean> {
  if (!model || !model.modelTrained) {
    return false;
  }
  // 加载训练好的模型进行匹配
  const confidence = await predict(model.id, text);
  return confidence !== null && confidence >= model.threshold;
}
