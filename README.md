<div align="center">

<img src="app-icon.png" alt="logo" width="80">

<h1>TextGO</h1>

[![GitHub Release](https://img.shields.io/github/v/release/C5H12O5/TextGO?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KCTxwYXRoIGZpbGw9Im5vbmUiIHN0cm9rZT0iI0ZGRkZGRiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjIiIGQ9Ik00IDhWNmEyIDIgMCAwIDEgMi0yaDJNNCAxNnYyYTIgMiAwIDAgMCAyIDJoMm04LTE2aDJhMiAyIDAgMCAxIDIgMnYybS00IDEyaDJhMiAyIDAgMCAwIDItMnYtMk04IDEyaDhNOCA5aDZtLTYgNmg0IiAvPgo8L3N2Zz4=&label=Release&style=flat&color=blue)](https://github.com/C5H12O5/TextGO/releases)
[![GitHub Stars](https://img.shields.io/github/stars/C5H12O5/TextGO?logo=github&label=Stars&style=flat&color=yellow)](https://github.com/C5H12O5/TextGO/stargazers)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.9.2-24C8D8.svg?logo=tauri)](https://tauri.app/)
[![Svelte Version](https://img.shields.io/badge/Svelte-v5.43.4-FF3E00.svg?logo=svelte)](https://svelte.dev/)
![macOS](https://img.shields.io/badge/macOS-333333.svg?logo=apple)
![Windows](https://img.shields.io/badge/Windows-0078D4.svg?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNTYiIGhlaWdodD0iMjU2IiB2aWV3Qm94PSIwIDAgMjU2IDI1NiI+Cgk8cGF0aCBmaWxsPSIjZmZmIiBkPSJNMTA0IDE0NHY1MS42NGE4IDggMCAwIDEtOCA4YTguNSA4LjUgMCAwIDEtMS40My0uMTNsLTY0LTExLjY0QTggOCAwIDAgMSAyNCAxODR2LTQwYTggOCAwIDAgMSA4LThoNjRhOCA4IDAgMCAxIDggOG0tMi44Ny04OS43OGE4IDggMCAwIDAtNi41Ni0xLjczbC02NCAxMS42NEE4IDggMCAwIDAgMjQgNzJ2NDBhOCA4IDAgMCAwIDggOGg2NGE4IDggMCAwIDAgOC04VjYwLjM2YTggOCAwIDAgMC0yLjg3LTYuMTRNMjA4IDEzNmgtODBhOCA4IDAgMCAwLTggOHY1Ny40NWE4IDggMCAwIDAgNi41NyA3Ljg4bDgwIDE0LjU0YTcuNiA3LjYgMCAwIDAgMS40My4xM2E4IDggMCAwIDAgOC04di03MmE4IDggMCAwIDAtOC04bTUuMTMtMTAyLjE0YTggOCAwIDAgMC02LjU2LTEuNzNsLTgwIDE0LjU1YTggOCAwIDAgMC02LjU3IDcuODdWMTEyYTggOCAwIDAgMCA4IDhoODBhOCA4IDAgMCAwIDgtOFY0MGE4IDggMCAwIDAtMi44Ny02LjE0IiBzdHJva2Utd2lkdGg9IjYuNSIgc3Ryb2tlPSIjZmZmIiAvPgo8L3N2Zz4=)

📖 English / [简体中文](README.zh-CN.md)

</div>

`TextGO` is a cross-platform global hotkey application for modern text workers. It automatically identifies selected text types and triggers custom actions, eliminating the need to memorize complex hotkeys.

## ✨ Core Features

- **Text Recognition**: Automatically identifies text type using regex, ML models, and NLP technologies.
- **Custom Actions**: Supports JavaScript/Python scripts and local AI conversations via Ollama.
- **Trainable Models**: Quickly train custom text recognition models with minimal samples.
- **Cross-Platform**: Built on Tauri framework, supports Windows and macOS.
- **Zero Memory Burden**: No need to memorize complex key combinations.

## 🚀 Use Cases

Here are the features you can achieve using `TextGO`:

- **Format Conversion**: Identify text with fixed rules and convert it to the format you need.
- **Text Generation**: Embed selected text into custom templates through scripts.
- **Word Translation**: Call local LLM to achieve AI translation.
- ...

## ⬇️ Getting Started

### Installation

Download the installer for your platform from [**GitHub Releases**](https://github.com/C5H12O5/TextGO/releases) and follow the installation instructions.

### FAQ

<details>
<summary>1. macOS shows "App is damaged and can't be opened."</summary>

<br>

_Run the following command in the terminal to resolve:_

```bash
sudo xattr -r -d com.apple.quarantine /Applications/TextGO.app
```

</details>

<details>
<summary>2. macOS shows "Apple can't check app for malicious software."</summary>

<br>

_Follow these steps to resolve:_

1. Open "System Settings" > "Privacy & Security"
2. Find the blocked application in the "Security" section
3. Click the "Open Anyway" button
4. Enter your login password and confirm

</details>

## 🎉 Acknowledgments

This project is built upon many excellent open source projects. We would like to express our sincere gratitude to all the developers and contributors of these projects.

For a complete list of third-party dependencies and their licenses, please see [**LICENSES.md**](LICENSES.md).

## 📄 License

This project is released under the [**MIT**](LICENSE) open source license.
