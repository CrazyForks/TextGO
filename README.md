<h1 align="center">TextGO</h1>

<div align="center">

[![GitHub Stars](https://img.shields.io/github/stars/C5H12O5/TextGO?logo=github&label=Stars&style=flat&color=yellow)](https://github.com/C5H12O5/TextGO/stargazers)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.8.5-24C8D8.svg?logo=tauri)](https://tauri.app/)
[![Svelte Version](https://img.shields.io/badge/Svelte-v5.39.6-FF3E00.svg?logo=svelte)](https://svelte.dev/)

📖 English / [简体中文](README.zh-CN.md)

</div>

`TextGO` is a cross-platform global hotkey application for modern text workers. It automatically identifies selected text types and triggers custom actions, eliminating the need to memorize complex hotkeys.

<div align="center">
  <img src="concept.jpg" alt="concept" width="1024">
</div>

## Core Features

- **Text Recognition**: Automatically identifies text type using regex, ML models, and NLP technologies.
- **Custom Actions**: Supports JavaScript/Python scripts and local AI conversations via Ollama.
- **Trainable Models**: Quickly train custom text recognition models with minimal samples.
- **Cross-Platform**: Built on Tauri framework, supports Windows and macOS.
- **Zero Memory Burden**: No need to memorize complex key combinations.

## Use Cases

Here are the features you can achieve using `TextGO`:

- **Format Conversion**: Identify text with fixed rules and convert it to the format you need.
- **Text Generation**: Embed selected text into custom templates through scripts.
- **Word Translation**: Call local LLM to achieve AI translation.
- ...

## Dependencies

| Name                                           | Description                                                                           |
| ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| [Tauri](https://tauri.app/)                    | Rust-based cross-platform desktop application framework providing native performance. |
| [Enigo](https://github.com/enigo-rs/enigo)     | Rust input simulation library for simulating mouse and keyboard input.                |
| [Svelte](https://svelte.dev/)                  | Modern frontend framework for building responsive user interfaces.                    |
| [DaisyUI](https://daisyui.com/)                | Semantic component library based on Tailwind CSS.                                     |
| [TailwindCSS](https://tailwindcss.com/)        | Utility-first CSS framework for rapidly building beautiful interfaces.                |
| [CodeMirror](https://codemirror.net/)          | Code editor component with syntax highlighting and auto-completion support.           |
| [franc](https://github.com/wooorm/franc)       | Natural language detection library for identifying human languages from text.         |
| [Guesslang](https://github.com/yoeo/guesslang) | Machine learning library for detecting programming languages from code snippets.      |
| [TensorFlow.js](https://www.tensorflow.org/js) | Browser-side machine learning supporting text classification model training.          |
| [Ollama](https://ollama.com/)                  | Local LLM runtime providing AI conversation capabilities.                             |

## License

This project is released under the [MIT](LICENSE) open source license.
