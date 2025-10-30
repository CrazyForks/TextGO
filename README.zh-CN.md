<div align="center">

<img src="app-icon.png" alt="logo" width="64">

<h1>TextGO</h1>

[![GitHub Release](https://img.shields.io/github/v/release/C5H12O5/TextGO?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KCTxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iI0ZGRkZGRiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjIiIGQ9Ik00IDhWNmEyIDIgMCAwIDEgMi0yaDJNNCAxNnYyYTIgMiAwIDAgMCAyIDJoMm04LTE2aDJhMiAyIDAgMCAxIDIgMnYybS00IDEyaDJhMiAyIDAgMCAwIDItMnYtMk04IDEyaDhNOCA5aDZtLTYgNmg0IiAvPgo8L3N2Zz4=&label=Release&style=flat&color=blue)](https://github.com/C5H12O5/TextGO/releases)
[![GitHub Stars](https://img.shields.io/github/stars/C5H12O5/TextGO?logo=github&label=Stars&style=flat&color=yellow)](https://github.com/C5H12O5/TextGO/stargazers)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.9.2-24C8D8.svg?logo=tauri)](https://tauri.app/)
[![Svelte Version](https://img.shields.io/badge/Svelte-v5.43.2-FF3E00.svg?logo=svelte)](https://svelte.dev/)

📖 [English](README.md) / 简体中文

</div>

`TextGO` 是一款跨平台的全局快捷键应用，专为现代文字工作者设计。它可以通过识别您选中的文本类型并触发相应的自定义动作，从而减少记忆快捷键的负担，提升工作效率。

## ✨ 核心特性

- **文本识别**：基于正则表达式、机器学习模型和自然语言处理技术，自动识别选中文本的类型
- **自定义动作**：支持 JavaScript 和 Python 脚本执行，以及基于 Ollama 的本地 AI 对话
- **可训练模型**：通过输入少量样本，快速训练专属的文本类型识别模型
- **跨平台兼容**：基于 Tauri 框架，支持 Windows 和 macOS
- **零记忆负担**：告别传统快捷键应用中需要记忆大量组合键的痛点

## 🚀 应用场景

以下是使用 `TextGO` 可以实现的功能：

- **格式转换**：识别具有固定规则的文本并将其转换成您需要的格式
- **文本生成**：通过脚本将选中文本嵌入到自定义的模板中
- **划词翻译**：调用本地 LLM 实现 AI 翻译
- ...

## ⬇️ 安装方法

从 [GitHub Releases](https://github.com/C5H12O5/TextGO/releases) 下载对应平台的安装包，按照安装说明进行安装后即可使用。

#### 常见问题

<details>
<summary>1. macOS 安装后打开提示“App已损坏，无法打开。”</summary>

<br>

_可以在终端运行以下命令解决：_

```bash
sudo xattr -r -d com.apple.quarantine /Applications/TextGO.app
```

</details>

## 🛠️ 依赖项目

| 名称                                           | 说明                                         |
| ---------------------------------------------- | -------------------------------------------- |
| [Tauri](https://tauri.app/)                    | 基于 Rust 的跨平台桌面应用框架，提供原生性能 |
| [Enigo](https://github.com/enigo-rs/enigo)     | Rust 输入模拟库，用于模拟鼠标和键盘输入      |
| [Svelte](https://svelte.dev/)                  | 现代前端框架，构建响应式用户界面             |
| [DaisyUI](https://daisyui.com/)                | 基于 Tailwind CSS 的语义化组件库             |
| [TailwindCSS](https://tailwindcss.com/)        | 实用优先的 CSS 框架，快速构建美观界面        |
| [CodeMirror](https://codemirror.net/)          | 代码编辑器组件，支持语法高亮和自动完成       |
| [franc](https://github.com/wooorm/franc)       | 自然语言检测库，用于识别文本的人类语言       |
| [Guesslang](https://github.com/yoeo/guesslang) | 机器学习库，用于从代码片段检测编程语言       |
| [TensorFlow.js](https://www.tensorflow.org/js) | 浏览器端机器学习，支持文本分类模型训练       |
| [Ollama](https://ollama.com/)                  | 本地 LLM 运行时，提供 AI 对话能力            |

## 📄 开源协议

本项目基于 [MIT](LICENSE) 开源协议发布
