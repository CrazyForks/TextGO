import * as tf from '@tensorflow/tfjs';

/**
 * 模型存储键名常量
 */
const STORAGE = {
  CLASSIFIER: 'sctc',
  CONFIG: 'sctc_config',
  TOKENIZER: 'sctc_tokenizer'
} as const;

/**
 * 模型缓存接口
 */
interface ModelCache {
  model: tf.LayersModel;
  tokenizer: Map<string, number>;
  config: {
    maxSequenceLength: number;
    embeddingDim: number;
    modelTrained: boolean;
    tokenizerSize: number;
  };
  lastUsed: number; // 最后使用时间，用于缓存清理
}

/**
 * 全局模型缓存 Map
 * key: 模型ID, value: 模型缓存对象
 */
const MODEL_CACHE = new Map<string, ModelCache>();

/**
 * 基于合成负样本的文本分类器
 *
 * 功能说明:
 * - 只需要提供正样本数据即可训练二分类模型
 * - 自动生成负样本来补充训练数据
 * - 支持多种文本模式识别（车牌号、ISBN、身份证等）
 * - 返回0-1之间的置信度分数
 *
 * 实现原理:
 * 1. 特征工程: 提取字符级、词级、模式、位置等多维特征
 * 2. 负样本生成: 通过随机化和噪声添加生成合成负样本
 * 3. 神经网络: 嵌入层 + 全局平均池化 + 全连接层的轻量级架构
 * 4. 二分类训练: 使用正样本(标签1) + 负样本(标签0)进行监督学习
 *
 * 技术特点:
 * - 支持多模型管理（通过ID区分不同用途的分类器）
 * - 本地存储持久化（模型、分词器、配置自动保存）
 * - 小数据集友好（最少3个正样本即可训练）
 * - 快速训练推理（轻量级架构，适合浏览器环境）
 */
export class SingleClassTextClassifier {
  private model: tf.LayersModel | null = null;
  private tokenizer: Map<string, number> = new Map();
  private maxSequenceLength = 50; // 缩短序列长度，适合少量样本
  private embeddingDim = 32; // 嵌入维度
  private modelTrained = false;
  private id: string; // 模型ID

  // 创建一个新的分类器实例
  constructor(id: string) {
    this.id = id;
  }

  // 训练单类别文本分类模型
  async trainModel(positiveTrainingData: string[] | string) {
    console.debug('准备单类别训练数据...');

    // 0. 验证和预处理训练数据
    const processedData = SingleClassTextClassifier.validateTrainingData(positiveTrainingData);
    if (!processedData) {
      throw new Error('训练数据格式无效或没有足够的样本');
    }

    try {
      // 1. 构建词汇表
      this.buildVocabulary(processedData);

      // 2. 生成负样本并准备训练数据
      const { inputs, labels } = this.prepareTrainingData(processedData);

      console.debug('输入形状:', inputs.shape);
      console.debug('标签形状:', labels.shape);
      console.debug('输入数据类型:', inputs.dtype);
      console.debug('标签数据类型:', labels.dtype);

      // 3. 创建模型
      this.model = this.createModel();

      // 4. 训练
      console.debug('开始训练单类别模型...');
      const history = await this.model.fit(inputs, labels, {
        epochs: 50, // 增加训练轮数
        batchSize: 8, // 小批量训练
        validationSplit: 0.2,
        shuffle: true,
        verbose: 1,
        callbacks: {
          onEpochEnd: (epoch, logs) => {
            console.debug(`轮次 ${epoch + 1}: 损失=${logs?.loss?.toFixed(4)}, 准确率=${logs?.acc?.toFixed(4)}`);
            if (logs?.val_loss) {
              console.debug(`  验证损失=${logs.val_loss.toFixed(4)}, 验证准确率=${logs?.val_acc?.toFixed(4)}`);
            }
          }
        }
      });

      // 5. 清理张量内存
      inputs.dispose();
      labels.dispose();

      // 6. 保存模型和分词器
      this.modelTrained = true;
      await this.saveModel();

      console.debug('单类别模型训练成功!');
      return history;
    } catch (error) {
      console.error('训练失败:', error);
      this.debugInfo();
      throw error;
    }
  }

  // 创建单类别分类模型
  private createModel(): tf.LayersModel {
    console.debug('创建单类别模型，词汇表大小:', this.tokenizer.size + 1);

    const model = tf.sequential({
      layers: [
        // 嵌入层
        tf.layers.embedding({
          inputDim: this.tokenizer.size + 1,
          outputDim: this.embeddingDim,
          inputLength: this.maxSequenceLength
        }),

        // 全局平均池化
        tf.layers.globalAveragePooling1d(),

        // 隐藏层
        tf.layers.dense({ units: 16, activation: 'relu' }),
        tf.layers.dropout({ rate: 0.3 }),

        // 输出层: 单个神经元，sigmoid激活函数用于二分类
        tf.layers.dense({
          units: 1,
          activation: 'sigmoid'
        })
      ]
    });

    // 使用二分类损失函数
    model.compile({
      optimizer: tf.train.adam(0.001),
      loss: 'binaryCrossentropy',
      metrics: ['accuracy']
    });

    return model;
  }

  // 构建词汇表（只处理正向样本）
  private buildVocabulary(positiveData: string[]) {
    const vocabulary = new Set<string>();

    // 提取特征词汇
    positiveData.forEach((text) => {
      const tokens = this.tokenizeText(text);
      tokens.forEach((token) => vocabulary.add(token));
    });

    // 构建映射
    let tokenIndex = 1; // 0 保留给未知词
    vocabulary.forEach((token) => {
      this.tokenizer.set(token, tokenIndex++);
    });

    console.debug(`词汇表大小: ${this.tokenizer.size}`);
  }

  // 通用文本标记化（适用于任意类型的文本模式）
  private tokenizeText(text: string): string[] {
    // 提取模式特征
    const patternFeatures = this.extractPatternFeatures(text);

    // 多粒度标记化
    const tokens = new Set<string>();

    // 1. 字符级特征
    const charFeatures = this.extractCharacterFeatures(text);
    charFeatures.forEach((feature) => tokens.add(feature));

    // 2. N-gram特征（字符级）
    const charNgrams = this.extractCharNgrams(text, 2, 4);
    charNgrams.forEach((ngram) => tokens.add(ngram));

    // 3. 词级特征
    const wordTokens = this.extractWordTokens(text);
    wordTokens.forEach((token) => tokens.add(token));

    // 4. 模式特征
    patternFeatures.forEach((feature) => tokens.add(feature));

    // 5. 位置特征
    const positionFeatures = this.extractPositionFeatures(text);
    positionFeatures.forEach((feature) => tokens.add(feature));

    return Array.from(tokens);
  }

  // 提取通用模式特征（适用于各种文本类型）
  private extractPatternFeatures(text: string): string[] {
    const features: string[] = [];

    // 长度特征
    if (text.length <= 5) features.push('VERY_SHORT');
    else if (text.length <= 10) features.push('SHORT');
    else if (text.length <= 20) features.push('MEDIUM');
    else if (text.length <= 50) features.push('LONG');
    else features.push('VERY_LONG');

    // 数字模式
    if (/^\d+$/.test(text)) features.push('ALL_DIGITS');
    if (/^\d{4}-\d{2}-\d{2}$/.test(text)) features.push('DATE_FORMAT');
    if (/^\d{4}\d{2}\d{2}$/.test(text)) features.push('DATE_COMPACT');
    if (/^\d{13}$/.test(text)) features.push('ISBN13');
    if (/^\d{10}$/.test(text)) features.push('ISBN10');
    if (/^\d{15}$/.test(text)) features.push('ID_15_DIGITS');
    if (/^\d{18}$/.test(text)) features.push('ID_18_DIGITS');

    // 车牌号模式（中国）
    if (/^[京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领][A-Z][A-Z0-9]{5}$/.test(text)) {
      features.push('CHINA_LICENSE_PLATE');
    }
    if (/^[A-Z]{2,3}\d{3,4}[A-Z]?$/.test(text)) features.push('LICENSE_PLATE_PATTERN');

    // 字母数字混合模式
    if (/^[A-Z0-9-]+$/.test(text.toUpperCase())) features.push('ALPHANUMERIC_DASH');
    if (/^[A-Z0-9]+$/.test(text.toUpperCase())) features.push('ALPHANUMERIC');
    if (/^[A-Z]+\d+$/.test(text.toUpperCase())) features.push('LETTERS_THEN_DIGITS');
    if (/^\d+[A-Z]+$/.test(text.toUpperCase())) features.push('DIGITS_THEN_LETTERS');

    // 分隔符模式
    if (text.includes('-')) features.push('HAS_DASH');
    if (text.includes('_')) features.push('HAS_UNDERSCORE');
    if (text.includes('.')) features.push('HAS_DOT');
    if (text.includes('/')) features.push('HAS_SLASH');
    if (text.includes(':')) features.push('HAS_COLON');
    if (text.includes(' ')) features.push('HAS_SPACE');

    // 大小写模式
    if (/^[A-Z]+$/.test(text)) features.push('ALL_UPPERCASE');
    if (/^[a-z]+$/.test(text)) features.push('ALL_LOWERCASE');
    if (/^[A-Z][a-z]+$/.test(text)) features.push('TITLE_CASE');
    if (/[A-Z]/.test(text) && /[a-z]/.test(text)) features.push('MIXED_CASE');

    // 重复字符模式
    if (/(.)\1{2,}/.test(text)) features.push('HAS_REPEATED_CHARS');
    if (/^(.+)\1+$/.test(text)) features.push('REPEATING_PATTERN');

    // 特殊字符
    if (/[!@#$%^&*(),.?":{}|<>]/.test(text)) features.push('HAS_SPECIAL_CHARS');
    if (/^[\u4e00-\u9fa5]+$/.test(text)) features.push('ALL_CHINESE');
    if (/[\u4e00-\u9fa5]/.test(text)) features.push('HAS_CHINESE');

    return features;
  }

  // 提取字符级特征
  private extractCharacterFeatures(text: string): string[] {
    const features: string[] = [];
    const chars = text.toLowerCase().split('');

    // 字符类型统计
    const digitCount = chars.filter((c) => /\d/.test(c)).length;
    const letterCount = chars.filter((c) => /[a-z]/.test(c)).length;
    const spaceCount = chars.filter((c) => c === ' ').length;
    const specialCount = chars.filter((c) => !/[a-z0-9\s]/.test(c)).length;

    // 比例特征
    const total = text.length;
    if (digitCount / total > 0.5) features.push('MOSTLY_DIGITS');
    if (letterCount / total > 0.5) features.push('MOSTLY_LETTERS');
    if (spaceCount / total > 0.1) features.push('MANY_SPACES');
    if (specialCount / total > 0.1) features.push('MANY_SPECIAL');

    // 首尾字符
    if (text.length > 0) {
      const first = text[0];
      const last = text[text.length - 1];

      if (/\d/.test(first)) features.push('STARTS_WITH_DIGIT');
      if (/[A-Za-z]/.test(first)) features.push('STARTS_WITH_LETTER');
      if (/\d/.test(last)) features.push('ENDS_WITH_DIGIT');
      if (/[A-Za-z]/.test(last)) features.push('ENDS_WITH_LETTER');
    }

    return features;
  }

  // 提取字符N-gram特征
  private extractCharNgrams(text: string, minN: number, maxN: number): string[] {
    const ngrams: string[] = [];
    const normalizedText = text.toLowerCase();

    for (let n = minN; n <= maxN; n++) {
      for (let i = 0; i <= normalizedText.length - n; i++) {
        const ngram = normalizedText.substring(i, i + n);
        ngrams.push(`NGRAM_${n}_${ngram}`);
      }
    }

    return ngrams;
  }

  // 提取词级特征
  private extractWordTokens(text: string): string[] {
    const tokens: string[] = [];

    // 按各种分隔符分割
    const words = text
      .toLowerCase()
      .split(/[\s\-_\.\/\\:,;!?]+/)
      .filter((word) => word.length > 0);

    words.forEach((word) => {
      if (word.length <= 15) {
        // 限制词长度避免过长token
        tokens.push(`WORD_${word}`);
      }
    });

    // 词数特征
    if (words.length === 1) tokens.push('SINGLE_WORD');
    else if (words.length <= 3) tokens.push('FEW_WORDS');
    else if (words.length <= 10) tokens.push('MANY_WORDS');
    else tokens.push('VERY_MANY_WORDS');

    return tokens;
  }

  // 提取位置特征
  private extractPositionFeatures(text: string): string[] {
    const features: string[] = [];

    // 数字位置特征
    const digitPositions = [];
    for (let i = 0; i < text.length; i++) {
      if (/\d/.test(text[i])) {
        digitPositions.push(i);
      }
    }

    if (digitPositions.length > 0) {
      const firstDigit = digitPositions[0];
      const lastDigit = digitPositions[digitPositions.length - 1];

      if (firstDigit === 0) features.push('DIGITS_AT_START');
      if (lastDigit === text.length - 1) features.push('DIGITS_AT_END');
      if (firstDigit > 0 && lastDigit < text.length - 1) features.push('DIGITS_IN_MIDDLE');
    }

    // 连续数字段
    const digitSegments = text.match(/\d+/g) || [];
    digitSegments.forEach((segment) => {
      const len = segment.length;
      if (len === 2) features.push('TWO_DIGIT_SEGMENT');
      else if (len === 3) features.push('THREE_DIGIT_SEGMENT');
      else if (len === 4) features.push('FOUR_DIGIT_SEGMENT');
      else if (len >= 5) features.push('LONG_DIGIT_SEGMENT');
    });

    return features;
  }

  // 准备单类别训练数据
  private prepareTrainingData(positiveData: string[]) {
    const sequences: number[][] = [];
    const labels: number[] = [];

    // 处理正向样本
    positiveData.forEach((text) => {
      const tokens = this.tokenizeText(text);
      const sequence = tokens.map((token) => this.tokenizer.get(token) || 0).slice(0, this.maxSequenceLength);

      // 填充或截断到固定长度
      while (sequence.length < this.maxSequenceLength) {
        sequence.push(0);
      }

      sequences.push(sequence);
      labels.push(1); // 正向样本标记为1
    });

    // 生成负向样本（通过随机化和噪声添加）
    const negativeCount = Math.min(positiveData.length, 20); // 限制负样本数量
    for (let i = 0; i < negativeCount; i++) {
      const negativeSequence = this.generateNegativeSample();
      sequences.push(negativeSequence);
      labels.push(0); // 负向样本标记为0
    }

    // 转换为张量
    const inputTensor = tf.tensor2d(sequences, [sequences.length, this.maxSequenceLength], 'float32');
    const labelsTensor = tf.tensor1d(labels, 'float32');

    console.debug('创建张量:');
    console.debug('输入张量形状:', inputTensor.shape, '数据类型:', inputTensor.dtype);
    console.debug('标签张量形状:', labelsTensor.shape, '数据类型:', labelsTensor.dtype);
    console.debug('正样本:', positiveData.length, '负样本:', negativeCount);

    return {
      inputs: inputTensor,
      labels: labelsTensor
    };
  }

  // 生成负样本
  private generateNegativeSample(): number[] {
    const sequence: number[] = [];
    const vocabSize = this.tokenizer.size;

    // 生成随机序列
    for (let i = 0; i < this.maxSequenceLength; i++) {
      if (Math.random() < 0.3) {
        // 30% 概率添加随机词汇
        sequence.push(Math.floor(Math.random() * vocabSize) + 1);
      } else {
        // 70% 概率填充为0
        sequence.push(0);
      }
    }

    return sequence;
  }

  // 预测文本是否属于目标类别
  predict(text: string): number {
    if (!this.model || !this.modelTrained) {
      console.warn('模型未加载或未训练。模型存在:', !!this.model, '模型已训练:', this.modelTrained);
      return 0;
    }

    const tokens = this.tokenizeText(text);
    console.debug('提取的Token:', tokens.slice(0, 10)); // 调试信息

    const sequence = tokens.map((token) => this.tokenizer.get(token) || 0).slice(0, this.maxSequenceLength);
    console.debug('Token序列:', sequence.slice(0, 10)); // 调试信息
    console.debug('序列中非零Token数量:', sequence.filter((x) => x > 0).length); // 调试信息

    // 填充到固定长度
    while (sequence.length < this.maxSequenceLength) {
      sequence.push(0);
    }

    // 检查序列是否全为0
    const nonZeroCount = sequence.filter((x) => x > 0).length;
    if (nonZeroCount === 0) {
      console.warn('所有Token都映射为0。这表明分词器不匹配。');
      console.debug('输入文本:', text);
      console.debug('提取的Token:', tokens.slice(0, 10));
      console.debug('分词器大小:', this.tokenizer.size);
      console.debug('分词器条目示例:', Array.from(this.tokenizer.entries()).slice(0, 5));

      // 尝试找到至少一个匹配的token
      const matchingTokens = tokens.filter((token) => this.tokenizer.has(token));
      console.debug('找到的匹配Token:', matchingTokens.slice(0, 5));

      if (matchingTokens.length === 0) {
        console.error('未找到匹配的Token。模型和输入文本完全不兼容。');
        return 0;
      }
    }

    try {
      // 使用 float32 保持与训练时的一致性
      const input = tf.tensor2d([sequence], [1, this.maxSequenceLength], 'float32');
      const prediction = this.model.predict(input) as tf.Tensor;
      const confidence = prediction.dataSync()[0]; // sigmoid输出的概率值

      console.debug('原始预测置信度:', confidence); // 调试信息

      // 清理内存
      input.dispose();
      prediction.dispose();

      return confidence;
    } catch (error) {
      console.error('预测错误:', error);
      return 0;
    }
  }

  // 保存模型
  async saveModel() {
    if (!this.model) {
      console.warn('没有模型可保存');
      return;
    }

    try {
      const storageKey = `${STORAGE.CLASSIFIER}_${this.id}`;
      await this.model.save(`localstorage://${storageKey}`);

      // 保存分词器和配置
      const tokenizerData = Array.from(this.tokenizer.entries());

      localStorage.setItem(`${STORAGE.TOKENIZER}_${this.id}`, JSON.stringify(tokenizerData));

      const config = {
        maxSequenceLength: this.maxSequenceLength,
        embeddingDim: this.embeddingDim,
        modelTrained: this.modelTrained,
        tokenizerSize: this.tokenizer.size
      };

      localStorage.setItem(`${STORAGE.CONFIG}_${this.id}`, JSON.stringify(config));

      // 添加到缓存
      MODEL_CACHE.set(this.id, {
        model: this.model,
        tokenizer: new Map(this.tokenizer),
        config: { ...config },
        lastUsed: Date.now()
      });

      console.debug('模型保存成功并已缓存');
      console.debug('分词器大小:', this.tokenizer.size);
    } catch (error) {
      console.error('保存模型失败:', error);
      throw error;
    }
  }

  // 加载模型
  async loadModel(): Promise<boolean> {
    try {
      console.debug('尝试加载模型...');

      // 首先检查缓存
      const cached = MODEL_CACHE.get(this.id);
      if (cached) {
        console.debug('从缓存加载模型:', this.id);
        this.model = cached.model;
        this.tokenizer = new Map(cached.tokenizer);
        this.maxSequenceLength = cached.config.maxSequenceLength;
        this.embeddingDim = cached.config.embeddingDim;
        this.modelTrained = cached.config.modelTrained;

        // 更新最后使用时间
        cached.lastUsed = Date.now();
        return true;
      }

      // 缓存中没有，从localStorage加载
      // 检查本地存储中是否有模型数据
      const tokenizerData = localStorage.getItem(`${STORAGE.TOKENIZER}_${this.id}`);
      const configData = localStorage.getItem(`${STORAGE.CONFIG}_${this.id}`);

      if (!tokenizerData || !configData) {
        console.debug('在localStorage中未找到保存的模型数据');
        return false;
      }

      // 加载TensorFlow模型
      const storageKey = `${STORAGE.CLASSIFIER}_${this.id}`;
      this.model = await tf.loadLayersModel(`localstorage://${storageKey}`);
      console.debug('TensorFlow模型加载成功');

      // 恢复分词器
      const tokenizerEntries = JSON.parse(tokenizerData);
      this.tokenizer = new Map(tokenizerEntries);
      console.debug('分词器已恢复，大小:', this.tokenizer.size);

      // 恢复配置
      const config = JSON.parse(configData);
      this.maxSequenceLength = config.maxSequenceLength;
      this.embeddingDim = config.embeddingDim;
      this.modelTrained = config.modelTrained;

      console.debug('配置已恢复:');
      console.debug('  最大序列长度:', this.maxSequenceLength);
      console.debug('  嵌入维度:', this.embeddingDim);
      console.debug('  模型已训练:', this.modelTrained);

      // 验证数据完整性
      if (config.tokenizerSize && config.tokenizerSize !== this.tokenizer.size) {
        console.warn('分词器大小不匹配。期望:', config.tokenizerSize, '实际:', this.tokenizer.size);
      }

      // 添加到缓存
      MODEL_CACHE.set(this.id, {
        model: this.model,
        tokenizer: new Map(this.tokenizer),
        config: {
          maxSequenceLength: this.maxSequenceLength,
          embeddingDim: this.embeddingDim,
          modelTrained: this.modelTrained,
          tokenizerSize: this.tokenizer.size
        },
        lastUsed: Date.now()
      });

      // 测试模型是否正常工作
      const testTokens = Array.from(this.tokenizer.keys()).slice(0, 3);
      if (testTokens.length > 0) {
        console.debug('加载的分词器示例Token:', testTokens);
      }

      console.debug('模型加载成功并已缓存:', this.id);
      return true;
    } catch (error) {
      console.error('加载模型失败:', error);

      // 清理可能部分加载的数据
      this.model = null;
      this.tokenizer.clear();
      this.modelTrained = false;

      return false;
    }
  }

  // 调试方法: 检查数据和模型状态
  debugInfo(): void {
    console.debug('=== 单类别文本分类器调试信息 ===');
    console.debug('模型ID:', this.id);
    console.debug('分词器大小:', this.tokenizer.size);
    console.debug('最大序列长度:', this.maxSequenceLength);
    console.debug('嵌入维度:', this.embeddingDim);
    console.debug('模型存在:', !!this.model);
    console.debug('模型已训练:', this.modelTrained);

    if (this.tokenizer.size > 0) {
      console.debug('示例Token:', Array.from(this.tokenizer.entries()).slice(0, 10));
    }

    console.debug('TensorFlow.js后端:', tf.getBackend());
    console.debug('内存使用:', tf.memory());
  }

  // 验证训练数据格式
  static validateTrainingData(data: string[] | string): string[] | null {
    let processedData: string[] = [];

    // 处理输入数据类型
    if (typeof data === 'string') {
      // 如果是字符串，按换行符分割
      processedData = data
        .split('\n')
        .map((line) => line.trim())
        .filter((line) => line.length > 0);
    } else if (Array.isArray(data)) {
      // 如果是数组，直接使用
      processedData = [...data];
    } else {
      console.error('训练数据必须是字符串或字符串数组');
      return null;
    }

    // 过滤无效文本
    let validData = processedData.filter((item) => {
      if (!item || typeof item !== 'string' || item.trim().length === 0) {
        console.debug('过滤无效文本:', item);
        return false;
      }
      return true;
    });

    // 去除重复数据
    validData = Array.from(new Set(validData));

    // 检查最终样本数量
    if (validData.length < 3) {
      console.error(`训练至少需要 3 个正样本，当前只有 ${validData.length} 个有效样本`);
      return null;
    }

    console.debug(`训练数据验证通过: ${validData.length} 个正样本`);
    return validData;
  }

  // 获取模型详细信息（包括存储大小和词汇表数量）
  static getModelInfo(id: string): {
    sizeKB: number;
    vocabulary: number;
  } {
    let sizeKB = 0;
    let vocabulary = 0;

    if (typeof window !== 'undefined' && window.localStorage) {
      try {
        // 获取词汇表大小
        const configKey = `${STORAGE.CONFIG}_${id}`;
        const configData = localStorage.getItem(configKey);
        if (configData) {
          const config = JSON.parse(configData);
          // 词汇表的大小和分词器大小相同
          vocabulary = config.tokenizerSize || 0;
        }
        // 计算 TensorFlow.js 模型文件大小
        let totalSize = 0;
        const modelPrefix = `tensorflowjs_models/${STORAGE.CLASSIFIER}_${id}`;
        for (let i = 0; i < localStorage.length; i++) {
          const key = localStorage.key(i);
          if (key && key.startsWith(modelPrefix)) {
            const value = localStorage.getItem(key);
            if (value) {
              // 计算 UTF-16 编码的字节大小（JavaScript 字符串是 UTF-16）
              totalSize += key.length * 2 + value.length * 2;
            }
          }
        }
        sizeKB = parseFloat((totalSize / 1024).toFixed(2));
      } catch (error) {
        console.error('获取模型信息失败:', error);
      }
    }

    return { sizeKB, vocabulary };
  }

  // 清除保存的模型数据
  static clearSavedModel(id: string): void {
    try {
      // 从缓存中移除并清理资源
      const cached = MODEL_CACHE.get(id);
      if (cached) {
        if (cached.model && typeof cached.model.dispose === 'function') {
          cached.model.dispose();
        }
        MODEL_CACHE.delete(id);
        console.debug('已从缓存清除模型:', id);
      }

      localStorage.removeItem(`${STORAGE.CONFIG}_${id}`);
      localStorage.removeItem(`${STORAGE.TOKENIZER}_${id}`);

      // 清除TensorFlow模型
      if (typeof window !== 'undefined' && window.localStorage) {
        const keys = [];
        for (let i = 0; i < localStorage.length; i++) {
          const key = localStorage.key(i);
          if (key && key.startsWith(`tensorflowjs_models/${STORAGE.CLASSIFIER}_${id}`)) {
            keys.push(key);
          }
        }
        keys.forEach((key) => localStorage.removeItem(key));
      }

      console.debug('已从localStorage清除保存的模型数据');
    } catch (error) {
      console.error('清除保存的模型失败:', error);
    }
  }

  // 清除当前实例的保存模型数据
  clearSavedModel(): void {
    SingleClassTextClassifier.clearSavedModel(this.id);
  }

  // 静态方法: 从缓存创建分类器实例
  static fromCache(id: string, cached: ModelCache): SingleClassTextClassifier {
    const classifier = new SingleClassTextClassifier(id);
    classifier.model = cached.model;
    classifier.tokenizer = new Map(cached.tokenizer);
    classifier.maxSequenceLength = cached.config.maxSequenceLength;
    classifier.embeddingDim = cached.config.embeddingDim;
    classifier.modelTrained = cached.config.modelTrained;
    return classifier;
  }
}

/**
 * 全局预测函数
 * 参数为模型ID和文本，实现直接从缓存Map中获取模型比对，没有的话再尝试从localStorage加载
 *
 * @param modelId - 模型ID
 * @param text - 要预测的文本
 * @returns 预测结果（正类概率）
 */
export async function predict(modelId: string, text: string): Promise<number | null> {
  try {
    // 首先尝试从缓存获取
    let cached = MODEL_CACHE.get(modelId);

    if (!cached) {
      // 缓存中没有，尝试加载
      console.debug('缓存中没有模型，尝试加载:', modelId);
      const classifier = new SingleClassTextClassifier(modelId);
      const loadSuccess = await classifier.loadModel();

      if (!loadSuccess) {
        console.warn('无法加载模型:', modelId);
        return null;
      }

      // 加载成功后再次从缓存获取
      cached = MODEL_CACHE.get(modelId);
      if (!cached) {
        console.error('模型加载后仍未在缓存中找到:', modelId);
        return null;
      }
    }

    // 更新最后使用时间
    cached.lastUsed = Date.now();

    // 检查模型是否已训练
    if (!cached.config.modelTrained) {
      console.warn('模型尚未训练:', modelId);
      return null;
    }

    // 使用缓存的模型进行预测
    const classifier = SingleClassTextClassifier.fromCache(modelId, cached);
    const result = classifier.predict(text);
    return result;
  } catch (error) {
    console.error('预测失败:', error);
    return null;
  }
}

/**
 * 清理缓存中过期的模型（超过指定时间未使用）
 *
 * @param maxAge - 最大存活时间（毫秒），默认1小时
 */
export function cleanupCache(maxAge: number = 60 * 60 * 1000): void {
  const now = Date.now();
  const toDelete: string[] = [];

  for (const [id, entry] of MODEL_CACHE.entries()) {
    if (now - entry.lastUsed > maxAge) {
      toDelete.push(id);
    }
  }

  for (const id of toDelete) {
    const cached = MODEL_CACHE.get(id);
    if (cached && cached.model && typeof cached.model.dispose === 'function') {
      cached.model.dispose();
    }
    MODEL_CACHE.delete(id);
    console.debug('清理过期缓存模型:', id);
  }

  if (toDelete.length > 0) {
    console.debug(`清理了 ${toDelete.length} 个过期模型`);
  }
}
