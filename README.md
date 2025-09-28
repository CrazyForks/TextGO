<h1 align="center">TextGO</h1>

<div align="center">

[![GitHub Stars](https://img.shields.io/github/stars/C5H12O5/TextGO?logo=github&label=Stars&style=flat&color=yellow)](https://github.com/C5H12O5/TextGO/stargazers)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.8.5-24C8D8.svg?logo=tauri)](https://tauri.app/)
[![Svelte Version](https://img.shields.io/badge/Svelte-v5.39.6-FF3E00.svg?logo=svelte)](https://svelte.dev/)

</div>

###### 📖 English / [简体中文](README.zh-CN.md)

TextGO is a cross-platform global hotkey application designed for modern text workers. It can automatically identify the type of text you select and trigger corresponding custom actions, thereby reducing the burden of memorizing hotkeys and improving work efficiency.

<div align="center">
  <img src="concept.jpg" alt="concept" width="1024">
</div>

## Core Features

- **Text Recognition**: Automatically identifies the type of selected text based on regular expressions, machine learning models, and natural language processing technologies
- **Custom Actions**: Supports JavaScript and Python script execution, as well as local AI conversations based on Ollama
- **Trainable Models**: Quickly train dedicated text type recognition models by inputting a small number of samples
- **Cross-Platform Compatibility**: Built on the Tauri framework, supporting Windows and macOS
- **Zero Memory Burden**: Say goodbye to the pain point of memorizing numerous key combinations in traditional hotkey applications

## Use Cases

Here are the features you can achieve using TextGO:

- **Format Conversion**: Identify text with fixed rules and convert it to the format you need
- **Text Generation**: Embed selected text into custom templates through scripts
- **Word Translation**: Call local LLM to achieve AI translation
- ...

## Dependencies

| Name                                               | Description                                                                          |
| -------------------------------------------------- | ------------------------------------------------------------------------------------ |
| **[Tauri](https://tauri.app/)**                    | Rust-based cross-platform desktop application framework providing native performance |
| **[Enigo](https://github.com/enigo-rs/enigo)**     | Rust input simulation library for simulating mouse and keyboard input                |
| **[Svelte](https://svelte.dev/)**                  | Modern frontend framework for building responsive user interfaces                    |
| **[DaisyUI](https://daisyui.com/)**                | Semantic component library based on Tailwind CSS                                     |
| **[TailwindCSS](https://tailwindcss.com/)**        | Utility-first CSS framework for rapidly building beautiful interfaces                |
| **[CodeMirror](https://codemirror.net/)**          | Code editor component with syntax highlighting and auto-completion support           |
| **[TensorFlow.js](https://www.tensorflow.org/js)** | Browser-side machine learning supporting text classification model training          |
| **[Ollama](https://ollama.com/)**                  | Local LLM runtime providing AI conversation capabilities                             |

## License

This project is released under the [MIT](LICENSE) open source license
