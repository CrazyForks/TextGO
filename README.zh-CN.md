<div align="center">

<img src="app-icon.png" alt="logo" width="80">

<h1>TextGO</h1>

[![GitHub Release](https://img.shields.io/github/v/release/C5H12O5/TextGO?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KCTxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iI0ZGRkZGRiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjIiIGQ9Ik00IDhWNmEyIDIgMCAwIDEgMi0yaDJNNCAxNnYyYTIgMiAwIDAgMCAyIDJoMm04LTE2aDJhMiAyIDAgMCAxIDIgMnYybS00IDEyaDJhMiAyIDAgMCAwIDItMnYtMk04IDEyaDhNOCA5aDZtLTYgNmg0IiAvPgo8L3N2Zz4=&label=Release&style=flat&color=blue)](https://github.com/C5H12O5/TextGO/releases)
[![GitHub Stars](https://img.shields.io/github/stars/C5H12O5/TextGO?logo=github&label=Stars&style=flat&color=yellow)](https://github.com/C5H12O5/TextGO/stargazers)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.9.2-24C8D8.svg?logo=tauri)](https://tauri.app/)
[![Svelte Version](https://img.shields.io/badge/Svelte-v5.43.2-FF3E00.svg?logo=svelte)](https://svelte.dev/)
![macOS](https://img.shields.io/badge/macOS-333333.svg?logo=apple)
![Windows](https://img.shields.io/badge/Windows-0078D4.svg?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNTYiIGhlaWdodD0iMjU2IiB2aWV3Qm94PSIwIDAgMjU2IDI1NiI+Cgk8cGF0aCBmaWxsPSIjZmZmIiBkPSJNMTA0IDE0NHY1MS42NGE4IDggMCAwIDEtOCA4YTguNSA4LjUgMCAwIDEtMS40My0uMTNsLTY0LTExLjY0QTggOCAwIDAgMSAyNCAxODR2LTQwYTggOCAwIDAgMSA4LThoNjRhOCA4IDAgMCAxIDggOG0tMi44Ny04OS43OGE4IDggMCAwIDAtNi41Ni0xLjczbC02NCAxMS42NEE4IDggMCAwIDAgMjQgNzJ2NDBhOCA4IDAgMCAwIDggOGg2NGE4IDggMCAwIDAgOC04VjYwLjM2YTggOCAwIDAgMC0yLjg3LTYuMTRNMjA4IDEzNmgtODBhOCA4IDAgMCAwLTggOHY1Ny40NWE4IDggMCAwIDAgNi41NyA3Ljg4bDgwIDE0LjU0YTcuNiA3LjYgMCAwIDAgMS40My4xM2E4IDggMCAwIDAgOC04di03MmE4IDggMCAwIDAtOC04bTUuMTMtMTAyLjE0YTggOCAwIDAgMC02LjU2LTEuNzNsLTgwIDE0LjU1YTggOCAwIDAgMC02LjU3IDcuODdWMTEyYTggOCAwIDAgMCA4IDhoODBhOCA4IDAgMCAwIDgtOFY0MGE4IDggMCAwIDAtMi44Ny02LjE0IiBzdHJva2Utd2lkdGg9IjYuNSIgc3Ryb2tlPSIjZmZmIiAvPgo8L3N2Zz4=)

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

## ⬇️ 使用说明

### 下载安装

从 [**GitHub Releases**](https://github.com/C5H12O5/TextGO/releases) 下载对应平台的安装包，按照安装说明进行安装后即可使用。

### 常见问题

<details>
<summary>1. macOS 安装后打开提示"App 已损坏，无法打开。"</summary>

<br>

_可以在终端运行以下命令解决：_

```bash
sudo xattr -r -d com.apple.quarantine /Applications/TextGO.app
```

</details>

<details>
<summary>2. macOS 安装后打开提示"Apple 无法检查 App 是否包含恶意软件。"</summary>

<br>

_可以按照以下步骤解决：_

1. 打开"系统设置" > "隐私与安全性"
2. 在"安全性"部分找到被阻止的应用
3. 点击"仍要打开"按钮
4. 输入你的登录密码后点击确认

</details>

## 🎉 特别鸣谢

本项目基于众多优秀的开源项目构建而成，在此向所有这些项目的开发者和贡献者表示衷心的感谢。

完整的第三方依赖列表及其开源协议请查看 [**LICENSES.md**](LICENSES.md) 文件。

## 📄 开源协议

本项目基于 [**MIT**](LICENSE) 开源协议发布。
