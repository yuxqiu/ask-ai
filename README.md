# Ask-AI

**Ask-AI** is a prototype desktop app that lets you chat with multiple LLMs simultaneously - **no API keys required**. It’s built in Rust using [Tao](https://github.com/tauri-apps/tao) and [Wry](https://github.com/tauri-apps/wry).

## 🖼️ Preview

| ![preview](img/preview.png) |
|:--:|
| *A preview of Ask-AI in action, chatting with multiple LLMs simultaneously.* |

## ✨ Features

- 🧠 Chat with multiple LLMs side by side (currently supports **ChatGPT** and **Grok**)
- 🔐 No API keys required - just run and chat
- ⚡ Lightweight native app built with Rust

## 🚀 Getting Started

### Option 1: Download Prebuilt Binaries

Prebuilt binaries are available for Windows, macOS, and Linux, supporting both x86_64 and ARM64 architectures. Visit the [GitHub Releases page](https://github.com/yuxqiu/ask-ai/releases) to download the latest release.

Available binaries:
- **Linux**: `ai-x86_64-unknown-linux-gnu`, `ai-aarch64-unknown-linux-gnu`
- **Windows**: `ai-x86_64-pc-windows-msvc.exe`, `ai-aarch64-pc-windows-msvc.exe`
- **macOS**: `ai-x86_64-apple-darwin`, `ai-aarch64-apple-darwin`

**Steps**:
1. Go to [Releases](https://github.com/yuxqiu/ask-ai/releases) and select the latest release.
2. Download the binary for your platform and architecture.
3. For Linux/macOS, make the binary executable:
   ```sh
   chmod +x ai-<target-triple>
   ```
4. Run the binary:
   - Linux/macOS: `./ai-<target-triple>`
   - Windows: Double-click `ai-<target-triple>.exe` or run `.\ai-<target-triple>.exe` in a terminal.

**Note**: On Linux, you may need to install dependencies as described in the [Tao Linux setup guide](https://github.com/tauri-apps/tao#linux) and [Wry Linux setup guide](https://github.com/tauri-apps/wry#linux-dependencies) to run the binary.

### Option 2: Build from Source

If you prefer to build from source or need to customize the app, follow these steps.

For Linux, begin by installing the required dependencies as described in the [Tao Linux setup guide](https://github.com/tauri-apps/tao#linux) and [Wry Linux setup guide](https://github.com/tauri-apps/wry#linux-dependencies).

Clone the repository and build the application:

```sh
git clone https://github.com/yuxqiu/ask-ai
cd ask-ai
cargo build --release
```

Run the app:

```sh
./target/release/ai
```

## 🛠️ Roadmap

- ✅ **Current**: Basic multi-LLM chat functionality with ChatGPT and Grok
- ✅ Cross-platform testing (Windows, macOS, Linux) and prebuilt release artifacts
- 🔜 Support for more LLMs, including local models via [Ollama](https://ollama.com/)
- 🔘 Support for file uploads, search, reasoning, and other user interaction buttons
- 🌍 Improve IME support for non-Latin scripts
- 🎨 UI redesign to better manage multiple conversations (browser-like tabbed interface)

## 📄 License

This project is licensed under the [MIT License](./LICENSE).

You're welcome to fork, extend, and contribute to help bring the roadmap to life!

## 🙏 Acknowledgments

Big thanks to the authors of the following projects:

- [AI-MultiPrompt-Extension](https://github.com/bsorrentino/AI-MultiPrompt-Extension/tree/main)