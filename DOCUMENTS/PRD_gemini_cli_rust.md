Product Requirements Document (PRD): Gemini CLI (Rust)
Project Name: gemini_cli_rust
Version: 1.0.0
Target Environment: Cross-platform (Windows, Linux, macOS)
Deployment Pattern: Single-binary CLI appliance

1. Executive Summary
gemini_cli_rust is a high-performance, command-line interface for Google's Gemini API. Built entirely in Rust, it bypasses the overhead of traditional Node.js or Python runtimes, delivering sub-millisecond startup times and memory-safe execution. The tool is designed with UNIX philosophy in mind, enabling seamless integration with system pipelines (stdin/stdout), real-time streaming, and self-contained zero-dependency deployment.

2. Goals & Objectives
Speed & Efficiency: Instantaneous execution with minimal memory footprint.

Portability: Compile to a single executable (under 10MB) requiring no external runtime environments.

Pipeline Integration: Natively accept piped input (e.g., cat logs.txt | gemini-rs "Analyze this").

Streaming Support: Real-time token rendering in the terminal to reduce perceived latency.

Security: Secure handling of API keys via environment variables or local .env configuration without hardcoding.

3. Technology Stack
Language: Rust (Edition 2021)

CLI Parsing: clap (v4 with derive feature)

Async Runtime: tokio (full)

Networking: reqwest (json, stream)

Serialization: serde, serde_json

Stream Handling: futures-util

Terminal Output: crossterm or colored (for syntax and role highlighting)

4. Feature Rollout Phases
Phase 1: Core Foundation (The One-Shot)
Authentication: Read GEMINI_API_KEY from system environment variables.

Basic Request: Send a single text prompt to the gemini-1.5-flash (or user-specified) endpoint.

Basic Response: Parse the JSON response and print the output text cleanly to the terminal.

Error Handling: Catch network timeouts, invalid API keys, and malformed JSON payloads gracefully.

Phase 2: The Streamer
Chunked Decoding: Implement reqwest streaming to process API chunks as they arrive.

Real-time Output: Use futures-util to print tokens to stdout immediately, matching the UX of premium AI terminal tools.

Flag Implementation: Add the --stream (or -s) flag via clap.

Phase 3: The UNIX Pipe (System Integration)
stdin Detection: Detect if data is being piped into the application (e.g., via is-terminal crate).

Context Wrapping: Concatenate the piped standard input with the user's CLI argument prompt.

Example: ls -la | gemini-rs "Explain this directory structure"

Phase 4: Session Persistence (Optional/Advanced)
Local State: Save multi-turn conversation history to a local ~/.gemini-rs/history.json file.

Chat Subcommand: Add a chat subcommand to initiate an interactive loop that reads previous history and maintains context across CLI invocations.

5. Command-Line Interface (CLI) Design
Defined via clap:

Bash
# Basic one-shot prompt
gemini-rs ask "What is the capital of France?"

# Streaming response
gemini-rs ask "Write a Rust script for parsing JSON" --stream

# Change model architecture
gemini-rs ask "Explain quantum physics" --model gemini-1.5-pro

# Piped input processing
cat server_error.log | gemini-rs ask "Find the root cause of this panic"
Proposed clap Struct Arguments:
[prompt]: (String) The primary instruction/query.

--stream / -s: (Bool) Enable real-time token streaming.

--model / -m: (String) Target specific model variant (default: gemini-1.5-flash).

--temp / -t: (Float) Set generation temperature (0.0 to 1.0).

6. Success Criteria
Build Validation: cargo build --release produces a clean, standalone executable.

Latency: The CLI initiates the network request within 10ms of the user pressing Enter.

Pipeline Success: Piped text files correctly inject into the API prompt structure without escaping errors.

Error Resilience: API failures (e.g., 503 Overloaded) output a clean, colored error message rather than a raw Rust panic.
