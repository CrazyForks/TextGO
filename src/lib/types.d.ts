/**
 * 将类型 T 转换为可为空的类型
 */
type Nullable<T> = T | null | undefined;

/**
 * 将类型 T 中的所有属性变为可选
 */
export type Optional<T> = {
  [P in keyof T]?: T[P] | null;
};

/**
 * 选项值
 */
export type OptionValue = string | number | boolean | null | undefined;

/**
 * 选项
 */
export type Option = {
  value: OptionValue;
  label: string;
  pattern?: RegExp;
  disabled?: boolean;
};

/**
 * 触发记录
 */
export type Log = {
  /** 记录标识 */
  id: string;
  /** 快捷键 */
  key: string;
  /** 文本类型 */
  caseLabel?: string;
  /** 触发动作 */
  actionLabel?: string;
  /** 动作类型 */
  actionType?: 'script' | 'prompt';
  /** 脚本语言 */
  scriptLang?: 'javascript' | 'python';
  /** 触发时间 */
  datetime: string;
  /** 剪贴版文本 */
  clipboard: string;
  /** 选中的文本 */
  selection: string;
  /** 执行结果 (脚本返回值/提示词) */
  result?: string;
  /** 系统提示词 */
  systemPrompt?: string;
  /** 响应内容 */
  response?: string;
  /** 是否在响应中 */
  streaming?: boolean;
  /** 是否离开页面 */
  leaving?: boolean;
};

/**
 * 快捷键
 */
export type Hotkey = {
  /** 快捷键标识 */
  id: string;
  /** 绑定的快捷键 */
  key: string;
  /** 绑定的文本类型 */
  case: string;
  caseLabel?: string;
  /** 要执行动作的标识 */
  action: string;
  actionLabel?: string;
};

/**
 * 脚本
 */
export type Script = {
  /** 脚本标识 */
  id: string;
  /** 脚本语言 */
  lang: 'javascript' | 'python';
  /** 脚本内容 */
  script: string;
  /** 是否静默执行 */
  quietMode?: boolean;
};

/**
 * 提示词
 */
export type Prompt = {
  /** 提示词标识 */
  id: string;
  /** 提示词内容 */
  prompt: string;
  /** 系统提示词 */
  systemPrompt?: string;
};

/**
 * 分类模型
 */
export type Model = {
  /** 分类模型标识 */
  id: string;
  /** 训练样本 */
  sample: string;
  /** 置信度阈值 */
  threshold: number;
  /** 模型是否已训练 */
  modelTrained?: boolean;
};
