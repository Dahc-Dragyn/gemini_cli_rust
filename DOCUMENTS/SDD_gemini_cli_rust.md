Software Design Document (SDD): Gemini CLI (Rust)
Project: gemini_cli_rust
Architecture Type: Asynchronous Monolithic CLI Binary
Language: Rust (Edition 2021)

1. System Architecture Overview
The application is designed as a stateless, single-pass execution binary, with optional stateful session management via local file caching. It utilizes an asynchronous event-driven architecture powered by tokio to manage non-blocking network I/O, ensuring that token streams from the Google Gemini API are immediately flushed to stdout without buffer pooling delays.

Core Architectural Principles
Zero-Allocation Parsing: Where possible, utilize string slices (&str) to minimize heap allocations during prompt ingestion.

Non-Blocking I/O: All network and file operations must be handled asynchronously to prevent thread-locking during API latency spikes.

Deterministic Execution: The program must exit cleanly with standard UNIX exit codes (0 for success, >0 for errors) to support integration into automated bash/shell scripts.

2. Module Design & Layout
The codebase will follow a standard Rust library-binary split to ensure testability and clean domain separation.

Directory Structure
Plaintext
src/
├── main.rs         # Binary entry point and global error handler
├── cli.rs          # Argument parsing and command routing (clap)
├── api/
│   ├── client.rs   # Reqwest HTTP client and connection pooling
│   └── models.rs   # Serde JSON serialization structs for Gemini API
├── io/
│   ├── stream.rs   # Asynchronous token processing and stdout flushing
│   └── pipe.rs     # Stdin detection and buffer reading
└── error.rs        # Custom `thiserror` definitions
Module Responsibilities
1. cli.rs (Command Router)
Defines the CLI schema using clap.

Responsible for prioritizing stdin pipes over positional arguments if both exist.

Parses model selection, temperature overrides, and stream flags.

2. api::client.rs (The Engine)
Initializes the reqwest::Client.

Injects the GEMINI_API_KEY into the x-goog-api-key header.

Routes requests to either generateContent (one-shot) or streamGenerateContent (chunked) endpoints.

3. api::models.rs (Data Contracts)
Defines strict, type-safe data structures mapping to the Gemini API schema.

Example Structs: GenerateContentRequest, Content, Part, StreamResponseChunk.

Uses #[serde(skip_serializing_if = "Option::is_none")] to keep payloads lightweight.

4. io::pipe.rs (The UNIX Bridge)
Detects if the terminal is running in an interactive TTY or receiving a piped stream.

If piped, reads the entire stdin buffer asynchronously and appends it to the user's prompt as context.

5. io::stream.rs (The Renderer)
Consumes the futures_core::stream::Stream from reqwest.

Deserializes server-sent events (SSE) or JSON array chunks on the fly.

Flushes standard output (std::io::Write::flush) character-by-character to create the "typing" effect.

3. Data Flow Diagram
Scenario: Streaming Piped Input
cat server.log | gemini-rs ask "Analyze this" --stream

Initialization: main.rs wakes the tokio runtime.

Ingestion: io::pipe.rs detects stdin data, reading the contents of server.log.

Construction: cli.rs combines the stdin data with the "Analyze this" prompt into a unified String.

Serialization: api::models.rs wraps the string into the expected Gemini JSON payload.

Transmission: api::client.rs opens a persistent HTTP/2 connection to Google's servers.

Execution Loop:

reqwest receives a byte chunk.

serde_json parses the chunk into a StreamResponseChunk.

io::stream.rs extracts the raw text.

stdout is locked, text is printed, and flush() is called.

Termination: Upon receiving the HTTP closure, the tokio runtime is dropped, and the program exits with code 0.

4. API Integration Details
Base URL: https://generativelanguage.googleapis.com/v1beta/models/

Target Endpoint: {model_name}:streamGenerateContent

Authentication: Passed via the URL query parameter ?key={GEMINI_API_KEY} or header x-goog-api-key.

Payload Schema Structure:

JSON
{
  "contents": [
    {
      "role": "user",
      "parts": [{ "text": "YOUR PROMPT HERE" }]
    }
  ],
  "generationConfig": {
    "temperature": 0.7
  }
}
5. Error Handling & Guardrails
To ensure this operates as a reliable systems tool, standard Rust unwrap() and panic!() macros are strictly forbidden in the execution path.

Missing API Key: If GEMINI_API_KEY is not found, the CLI will not panic. It will exit gracefully with a colorized error instructing the user how to set the environment variable.

Network Timeouts: The reqwest::Client will be configured with a strict 30-second connection timeout to prevent hanging the terminal if DNS resolution fails.

Rate Limiting (429): If the API returns a 429 Too Many Requests, the application will catch the status code and return a clean UNIX error code (exit 1) with a human-readable backoff warning.

6. Phase 1 Implementation Plan
Initialize cargo new gemini-rs.

Add dependencies to Cargo.toml.

Build the cli.rs argument parser.

Implement the api::models.rs structs.

Write the main.rs asynchronous one-shot execution (no streaming yet).

Test against the live API.
