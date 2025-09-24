import { alert } from '$lib/components';
import type { EventHandler } from 'svelte/elements';

/**
 * 可应用于表单输入的约束类型
 */
export type Constraint = Partial<{
  value: any; // eslint-disable-line
  name: string;
  type: string;
  required: boolean;
  autocomplete: AutoFill;
  min: string | number;
  max: string | number;
  step: string | number;
  minlength: number;
  maxlength: number;
  pattern: string;
  oninvalid: EventHandler<Event>;
}>;

/**
 * 约束构建器的类型
 */
export type ConstraintBuilder =
  | StringConstraintBuilder
  | PatternConstraintBuilder
  | NumberConstraintBuilder
  | DatetimeConstraintBuilder;

/**
 * 约束构建器的抽象类
 */
abstract class AbstractConstraintBuilder<T> {
  protected constraint: Constraint = {
    required: true
  };

  required(required: boolean): this {
    this.constraint.required = required;
    return this;
  }

  autocomplete(autocomplete: AutoFill): this {
    this.constraint.autocomplete = autocomplete;
    return this;
  }

  value(value: T): this {
    this.constraint.value = value;
    return this;
  }

  oninvalid(oninvalid: string | EventHandler<Event>): this {
    if (typeof oninvalid === 'string') {
      const message = oninvalid;
      oninvalid = () => alert({ level: 'error', message: message });
    }
    this.constraint.oninvalid = oninvalid;
    return this;
  }

  build(name: string): Constraint {
    this.constraint.name = name;
    return this.constraint;
  }
}

/**
 * 字符串约束的构建器
 */
class StringConstraintBuilder extends AbstractConstraintBuilder<string> {
  minlength(minlength: number): this {
    this.constraint.minlength = minlength;
    return this;
  }

  maxlength(maxlength: number): this {
    this.constraint.maxlength = maxlength;
    return this;
  }
}

/**
 * 带有正则表达式模式的字符串约束构建器
 */
class PatternConstraintBuilder extends StringConstraintBuilder {
  constructor(type: string) {
    super();
    this.constraint.type = type;
  }

  pattern(pattern: string): this {
    this.constraint.pattern = pattern;
    return this;
  }
}

/**
 * 数字约束的构建器
 */
class NumberConstraintBuilder extends AbstractConstraintBuilder<number> {
  constructor(type: string) {
    super();
    this.constraint.type = type;
  }

  min(min: number): this {
    this.constraint.min = min;
    return this;
  }

  max(max: number): this {
    this.constraint.max = max;
    return this;
  }

  step(step: number): this {
    this.constraint.step = step;
    return this;
  }
}

/**
 * 日期时间约束的构建器
 */
class DatetimeConstraintBuilder extends AbstractConstraintBuilder<string> {
  constructor(type: string) {
    super();
    this.constraint.type = type;
  }

  min(min: string): this {
    this.constraint.min = min;
    return this;
  }

  max(max: string): this {
    this.constraint.max = max;
    return this;
  }

  step(step: number | 'any'): this {
    this.constraint.step = step;
    return this;
  }
}

/**
 * 约束构建器工厂的映射
 */
const constraintBuilderFactories = {
  // textarea 元素
  textarea: () => new StringConstraintBuilder(),
  // 类型为 text、url、email、password 的 input 元素
  text: () => new PatternConstraintBuilder('text'),
  url: () => new PatternConstraintBuilder('url'),
  email: () => new PatternConstraintBuilder('email'),
  password: () => new PatternConstraintBuilder('password'),
  // 类型为 number、range 的 input 元素
  number: () => new NumberConstraintBuilder('number'),
  range: () => new NumberConstraintBuilder('range'),
  // 类型为 datetime-local、date、time、week、month 的 input 元素
  datetime: () => new DatetimeConstraintBuilder('datetime-local'),
  date: () => new DatetimeConstraintBuilder('date'),
  time: () => new DatetimeConstraintBuilder('time'),
  week: () => new DatetimeConstraintBuilder('week'),
  month: () => new DatetimeConstraintBuilder('month')
};

/**
 * 为表单输入创建约束模式
 *
 * @param use - 从约束构建器工厂中选择并配置约束构建器的函数
 * @returns 创建好的约束模式
 */
export function buildFormSchema<T extends Record<string, ConstraintBuilder>>(
  use: (arg: typeof constraintBuilderFactories) => T
): { [K in keyof T]: Constraint } {
  const builders = use(constraintBuilderFactories);
  const constraints = Object.entries(builders).map(([key, builder]) => [key, builder.build(key)]);
  return Object.fromEntries(constraints);
}
