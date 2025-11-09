import type { IconComponentProps } from 'phosphor-svelte';
import type { Component } from 'svelte';

/**
 * Convert all properties in type T to nullable
 */
export type Optional<T> = {
  [P in keyof T]?: T[P] | null;
};

/**
 * Convert type T itself to nullable
 */
type Nullable<T> = T | null | undefined;

/**
 * Option value
 */
export type OptionValue = string | number | boolean | null | undefined;

/**
 * Option
 */
export type Option = {
  /** option value */
  value: OptionValue;
  /** option label */
  label: string;
  /** whether disabled */
  disabled?: boolean;
  /** regular expression */
  pattern?: RegExp;
  /** option icon */
  icon?: Component<IconComponentProps>;
};

/**
 * Trigger record
 */
export type Entry = {
  /** record ID */
  id: string;
  /** triggered key */
  key: string;
  /** trigger time */
  datetime: string;
  /** clipboard text */
  clipboard: string;
  /** selected text */
  selection: string;
  /** text type */
  caseLabel?: string;
  /** triggered action */
  actionLabel?: string;
  /** action type */
  actionType?: 'script' | 'prompt';
  /** execution result (script return value/prompt) */
  result?: string;
  /** script language */
  scriptLang?: 'javascript' | 'python';
  /** silent mode */
  quietMode?: boolean;
  /** model provider */
  provider?: 'ollama' | 'lmstudio';
  /** model name */
  model?: string;
  /** system prompt */
  systemPrompt?: string;
  /** response content */
  response?: string;
};

/**
 * Rule
 */
export type Rule = {
  /** rule ID */
  id: string;
  /** bound shortcut key */
  key: string;
  /** bound text type */
  case: string;
  caseLabel?: string;
  /** ID of the action to execute */
  action: string;
  actionLabel?: string;
};

/**
 * Script
 */
export type Script = {
  /** script ID */
  id: string;
  /** script language */
  lang: 'javascript' | 'python';
  /** script content */
  script: string;
  /** silent mode */
  quietMode?: boolean;
};

/**
 * Prompt
 */
export type Prompt = {
  /** prompt ID */
  id: string;
  /** model provider */
  provider: 'ollama' | 'lmstudio';
  /** model name */
  model: string;
  /** prompt content */
  prompt: string;
  /** system prompt */
  systemPrompt?: string;
};

/**
 * Classification model
 */
export type Model = {
  /** classification model ID */
  id: string;
  /** training sample */
  sample: string;
  /** confidence threshold */
  threshold: number;
  /** whether model is trained */
  modelTrained?: boolean;
};

/**
 * Regular expression
 */
export type Regexp = {
  /** regex ID */
  id: string;
  /** regex pattern */
  pattern: string;
  /** regex flags */
  flags?: string;
};
