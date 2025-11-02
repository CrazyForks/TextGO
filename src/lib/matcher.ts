import { predict } from '$lib/classifier';
import { MODEL_MARK, REGEXP_MARK } from '$lib/constants';
import { m } from '$lib/paraglide/messages';
import { models, regexps } from '$lib/stores.svelte';
import type { Model, Option, Rule } from '$lib/types';
import { ModelOperations, type ModelResult } from '@vscode/vscode-languagedetection';
import { memoize } from 'es-toolkit/function';
import { franc } from 'franc-min';
import { CalendarDots, Clock, Folders, Globe, GlobeSimple, Key, Link, Mailbox, TextAa } from 'phosphor-svelte';

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
 * 常规识别选项
 */
export const GENERAL_CASES: Option[] = [
  {
    value: 'uuid',
    label: m.uuid(),
    icon: Key,
    pattern: /^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$/i
  },
  {
    value: 'guid',
    label: m.guid(),
    icon: Key,
    pattern: /^\{?[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\}?$/i
  },
  {
    value: 'url',
    label: m.url(),
    icon: Link,
    pattern: /^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9()@:%_+.~#?&/=]*$/
  },
  {
    value: 'email',
    label: m.email(),
    icon: Mailbox,
    pattern:
      /^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/i
  },
  {
    value: 'path',
    label: m.path(),
    icon: Folders,
    pattern: /^(?:[a-zA-Z]:\\[^<>:"|?*\n\r/]+(?:\\[^<>:"|?*\n\r/]+)*|~?\/[^<>:"|?*\n\r\\]+(?:\/[^<>:"|?*\n\r\\]+)*)$/
  },
  {
    value: 'ipv4',
    label: m.ipv4(),
    icon: GlobeSimple,
    pattern:
      /^(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/
  },
  {
    value: 'ipv6',
    label: m.ipv6(),
    icon: Globe,
    pattern:
      /^(?:(?:[0-9a-f]{1,4}:){7}[0-9a-f]{1,4}|(?:[0-9a-f]{1,4}:){1,7}:|(?:[0-9a-f]{1,4}:){1,6}:[0-9a-f]{1,4}|(?:[0-9a-f]{1,4}:){1,5}(?::[0-9a-f]{1,4}){1,2}|(?:[0-9a-f]{1,4}:){1,4}(?::[0-9a-f]{1,4}){1,3}|(?:[0-9a-f]{1,4}:){1,3}(?::[0-9a-f]{1,4}){1,4}|(?:[0-9a-f]{1,4}:){1,2}(?::[0-9a-f]{1,4}){1,5}|[0-9a-f]{1,4}:(?::[0-9a-f]{1,4}){1,6}|:(?:(?::[0-9a-f]{1,4}){1,7}|:)|fe80:(?::[0-9a-f]{0,4}){0,4}%[0-9a-z]+|::(?:ffff(?::0{1,4})?:)?(?:(?:25[0-5]|(?:2[0-4]|1?[0-9])?[0-9])\.){3}(?:25[0-5]|(?:2[0-4]|1?[0-9])?[0-9])|(?:[0-9a-f]{1,4}:){1,4}:(?:(?:25[0-5]|(?:2[0-4]|1?[0-9])?[0-9])\.){3}(?:25[0-5]|(?:2[0-4]|1?[0-9])?[0-9]))$/i
  },
  {
    value: 'iso8601',
    label: m.iso8601(),
    icon: CalendarDots,
    pattern:
      /^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|02-(?:0[1-9]|1\d|2[0-8])))T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:\.\d+)?(?:[+-](?:[01]\d|2[0-3]):[0-5]\d|Z)?$/
  },
  {
    value: 'timestamp',
    label: m.timestamp(),
    icon: Clock,
    // 2001 ~ 2286 年间10位秒级或13位毫秒级的 Unix 时间戳
    pattern: /^(?:[1-9]\d{9}|[1-9]\d{12})$/
  }
];

/**
 * 命名格式识别选项
 */
export const TEXT_CASES: Option[] = [
  {
    value: 'camel_case',
    label: m.camel_case(),
    icon: TextAa,
    pattern: /^[a-z][a-z0-9]*(?:[A-Z][a-z0-9]*)+$/
  },
  {
    value: 'pascal_case',
    label: m.pascal_case(),
    icon: TextAa,
    pattern: /^[A-Z]+[a-z0-9]+(?:[A-Z][a-z0-9]*)+$|^[A-Z]{2,}[a-z0-9]+$/
  },
  {
    value: 'snake_case',
    label: m.snake_case(),
    icon: TextAa,
    pattern: /^[a-z0-9]+(?:_[a-z0-9]+)+$/
  },
  {
    value: 'kebab_case',
    label: m.kebab_case(),
    icon: TextAa,
    pattern: /^[a-z0-9]+(?:-[a-z0-9]+)+$/
  },
  {
    value: 'lower_case',
    label: m.lower_case(),
    icon: TextAa,
    pattern: /^(?=.*[a-z])[a-z0-9]+(?: [a-z0-9]+)*$/
  },
  {
    value: 'upper_case',
    label: m.upper_case(),
    icon: TextAa,
    pattern: /^(?=.*[A-Z])[A-Z0-9]+(?: [A-Z0-9]+)*$/
  }
];

/**
 * 自然语言识别选项
 */
export const NATURAL_CASES: Option[] = [
  { value: 'cmn', label: m.lang_cmn() },
  { value: 'eng', label: m.lang_eng() },
  { value: 'jpn', label: m.lang_jpn() },
  { value: 'kor', label: m.lang_kor() },
  { value: 'rus', label: m.lang_rus() },
  { value: 'fra', label: m.lang_fra() },
  { value: 'deu', label: m.lang_deu() },
  { value: 'spa', label: m.lang_spa() },
  { value: 'por', label: m.lang_por() },
  { value: 'arb', label: m.lang_arb() }
];

/**
 * 编程语言识别选项
 */
export const PROGRAMMING_CASES: Option[] = [
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

// Memoized 查找函数
const findBuiltinCase = memoize((_case: string) => [...GENERAL_CASES, ...TEXT_CASES].find((c) => c.value === _case));
const findNaturalCase = memoize((_case: string) => NATURAL_CASES.find((c) => c.value === _case));
const findProgrammingCase = memoize((_case: string) => PROGRAMMING_CASES.find((c) => c.value === _case));

/**
 * 根据文本类型匹配要执行的快捷键动作
 *
 * @param text - 待匹配的文本
 * @param rules - 指定的规则列表
 * @returns 匹配到的规则对象，未匹配到则返回`null`
 */
export async function match(text: string, rules: Rule[]): Promise<Rule | null> {
  console.debug('获取待匹配模式:', rules.map((r) => r.case || '跳过').join('、'));
  let langDetected = false;
  let langDetectedResult: ModelResult[] = [];
  // 循环所有绑定的规则，找到第一个匹配的文本类型
  for (const rule of rules) {
    const _case = rule.case;
    // 空字符串表示不进行任何识别
    if (_case === '') {
      console.debug('%c跳过文本识别', 'color: rgba(0,0,0,0.5)');
      return rule;
    }
    if (!text) {
      continue;
    }

    // 内置正则匹配
    const builtin = findBuiltinCase(_case);
    if (builtin && builtin.pattern && builtin.pattern.test(text)) {
      console.debug('内置正则表达式匹配成功:', builtin.label);
      rule.caseLabel = builtin.label;
      return rule;
    }

    // 自然语言识别
    const natural = findNaturalCase(_case);
    if (natural) {
      try {
        if (franc(text, { minLength: 2 }) === _case) {
          console.debug('自然语言识别成功:', natural.label);
          rule.caseLabel = natural.label;
          return rule;
        }
      } catch (error) {
        console.error('自然语言识别失败:', error);
      }
    }

    // 编程语言识别
    const programming = findProgrammingCase(_case);
    if (programming) {
      try {
        if (!langDetected) {
          langDetectedResult = await modelOperations.runModel(text);
          langDetected = true;
          console.debug('编程语言识别结果:', langDetectedResult);
        }
        if (matchProgrammingCase(_case, langDetectedResult)) {
          console.debug('编程语言识别成功:', programming.label);
          rule.caseLabel = programming.label;
          return rule;
        }
      } catch (error) {
        console.error('编程语言识别失败:', error);
      }
    }

    // 自定义正则匹配
    if (_case.startsWith(REGEXP_MARK)) {
      const regexpId = _case.substring(REGEXP_MARK.length);
      const regexp = regexps.current.find((r) => r.id === regexpId);
      if (regexp && regexp.pattern) {
        try {
          const pattern = new RegExp(regexp.pattern, regexp.flags);
          if (pattern.test(text)) {
            console.debug('自定义正则表达式匹配成功:', regexp.id);
            rule.caseLabel = regexp.id;
            return rule;
          }
        } catch (error) {
          console.error('自定义正则表达式匹配失败:', error);
        }
      }
    }

    // 自定义模型识别
    if (_case.startsWith(MODEL_MARK)) {
      const modelId = _case.substring(MODEL_MARK.length);
      const model = models.current.find((m) => m.id === modelId);
      if (model) {
        try {
          if (await matchModelCase(model, text)) {
            console.debug('自定义模型识别成功:', model.id);
            rule.caseLabel = model.id;
            return rule;
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
function matchProgrammingCase(targetId: string, results: ModelResult[]): boolean {
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
