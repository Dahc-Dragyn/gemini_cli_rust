# ♊ Gemini CLI (Rust)

### *A High-Speed, Zero-Dependency terminal companion for Google's Gemini API*

**gemini_cli_rust** (compiled as `gemini-rs` or `gemini_cli_rust.exe`) is an ultra-fast, lightweight command-line tool built entirely in Rust. It compiles down to a **single portable binary (under 10MB)** with sub-millisecond startup times—perfect for local scripting, automated pipelines, or quick queries without opening a browser or waiting for heavy runtimes (like Node.js or Python) to spin up.

---

## ✨ Features

- 🚀 **Sub-Millisecond Startup:** Launches instantly with zero runtime execution delay.
- 📦 **Single Standalone Binary:** Zero dependencies. Copy the compiled `.exe` or binary and run it anywhere.
- 🌊 **Real-Time Streaming:** Streams response tokens to your terminal instantly using Google's Server-Sent Events (SSE).
- 🔗 **UNIX Pipeline Compositions:** Seamlessly accepts piped standard inputs (e.g. `cat error.log | gemini-rs ask "Find the bug"`).
- 🛡️ **Zero Panics:** Designed with robust systems engineering to catch network, JSON, or key configuration errors gracefully.
- 🔑 **Simple Setup:** Securely loads your API keys via environment variables or a local `.env` file.

---

## 📂 Project Architecture

```text
src/
├── main.rs         # Async entry point and global error handler
├── lib.rs          # Module declarations
├── cli.rs          # Argument parsing (clap v4)
├── error.rs        # Custom unified domain error mapping (thiserror)
├── api/
│   ├── client.rs   # Async HTTP connection pooling client (reqwest)
│   └── models.rs   # Type-safe JSON serialization/deserialization (serde)
└── io/
    ├── pipe.rs     # Non-blocking UNIX pipe reader
    └── stream.rs   # Real-time token streaming and rendering (futures)
```

---

## ⚡ Quick Setup (No Rust Experience Required!)

### Step 1: Secure Your API Key
1. Get a free Gemini API key from [Google AI Studio](https://aistudio.google.com/).
2. In the folder where you intend to run the CLI, create a file named `.env` and add your key:
   ```env
   GEMINI_API_KEY="YOUR_ACTUAL_API_KEY_HERE"
   ```
   *(Note: The repository is configured to ignore `.env` files automatically, so your key will never be accidentally committed to GitHub.)*

### Step 2: Build the Executable
If you have the Rust toolchain installed:
1. Open your terminal in this repository directory.
2. Run the production release build:
   ```bash
   cargo build --release
   ```
3. Your optimized executable will be generated at:
   `./target/release/gemini_cli_rust.exe` (on Windows) or `./target/release/gemini_cli_rust` (on macOS/Linux).

---

## 🚀 How to Use It (Examples)

Navigate to where your compiled binary resides (or add it to your system PATH) and test these command options:

### 1. Simple Prompt (One-Shot Response)
Ask standard questions directly:
```bash
.\gemini_cli_rust.exe ask "What is the capital of Japan?"
```

### 2. Streaming Output (Highly Recommended!)
Renders the response character-by-character as the model generates it, giving you instant answers:
```bash
.\gemini_cli_rust.exe ask "Write a story about a space hamster" --stream
```

### 3. Piping Input (UNIX Compositions)
This is the super-power of this CLI. You can pipe any command output, file, or script context directly into the tool, and append your question:

* **Debugging Code Snippets:**
  ```bash
  echo "fn main() { println!('Hello'); }" | .\gemini_cli_rust.exe ask "What's wrong with this Rust code?"
  ```
* **Analyzing Log Files:**
  ```bash
  cat server_error.log | .\gemini_cli_rust.exe ask "Identify the root cause of this stack trace" --stream
  ```
* **Explaining Directory Structures:**
  ```bash
  dir | .\gemini_cli_rust.exe ask "Explain what these files are"
  ```

---

## 🔧 Optional Flags

| Flag | Shortcut | Default | Description |
|---|---|---|---|
| `--model` | `-m` | `gemini-3.1-flash-lite` | Target model variation (e.g. `gemini-1.5-pro` or `gemini-3.1-flash-lite`) |
| `--stream` | `-s` | *False* | Toggles real-time streaming token render |
| `--temp` | `-t` | *None (0.7)* | Set temperature override (`0.0` for deterministic, `1.0` for creative) |

---

## 📋 System Requirements

- **OS:** Windows 10/11, macOS, or Linux.
- **Dependencies:** None. Runs 100% standalone.
- **Internet:** Required to call Google's API endpoints.

---

*Built with ❤️ in Rust for speed-obsessed developers.*
