Software Requirements Specification (SRS)
Project Name: gemini_cli_rust
Version: 1.0.0

1. Introduction
1.1 Purpose
This document specifies the functional and non-functional requirements for gemini_cli_rust, a command-line interface appliance that interacts directly with the Google Gemini API.

1.2 Scope
The software is a single-binary, compiled executable written in Rust. It takes user input via command-line arguments or standard input (stdin), transmits the payload asynchronously to the Gemini API, and returns the response to standard output (stdout).

2. Functional Requirements (FR)
2.1 Authentication & Configuration
FR-1.1: The system MUST read the API key from the GEMINI_API_KEY environment variable.

FR-1.2: If the environment variable is missing, the system MUST halt execution, print a human-readable error to stderr, and exit with code 1.

FR-1.3: The system MUST NOT hardcode or log the API key at any point during execution.

2.2 Command Line Interface (CLI)
FR-2.1: The system MUST parse arguments using the clap library.

FR-2.2: The system MUST support a primary ask subcommand requiring a string <prompt>.

FR-2.3: The system MUST support a --model (or -m) flag to override the default model (default: gemini-1.5-flash).

FR-2.4: The system MUST support a --stream (or -s) flag to trigger chunked responses.

2.3 Core API Execution (One-Shot)
FR-3.1: The system MUST construct a valid JSON payload matching the generateContent Gemini API schema.

FR-3.2: The system MUST transmit the payload over HTTPS using reqwest.

FR-3.3: The system MUST deserialize the JSON response and extract the text content safely.

FR-3.4: The system MUST print the final text to stdout and exit with code 0.

2.4 Streaming Execution
FR-4.1: When the --stream flag is active, the system MUST target the streamGenerateContent API endpoint.

FR-4.2: The system MUST process the incoming byte stream in real-time.

FR-4.3: The system MUST flush each decoded text chunk immediately to stdout without waiting for the full response to complete.

2.5 UNIX Pipeline Integration (stdin)
FR-5.1: The system MUST detect if data is being piped into the application via standard input (e.g., cat log.txt | gemini-rs).

FR-5.2: If stdin data is detected, the system MUST read the buffer entirely and prepend/append it to the user's CLI prompt argument before transmitting to the API.

3. Non-Functional Requirements (NFR)
3.1 Performance & Footprint
NFR-1.1: The compiled release binary MUST be under 15MB in size.

NFR-1.2: The system MUST initiate the HTTP network request within 50 milliseconds of binary execution (eliminating heavy runtime initialization).

3.2 Reliability & Error Handling
NFR-2.1: The system MUST NOT use raw unwrap() or panic!() in the main execution flow; all errors must be caught and handled.

NFR-2.2: On network timeout (>30 seconds), the system MUST exit gracefully with code 1.

NFR-2.3: On receiving an HTTP 429 (Too Many Requests), the system MUST output a clear rate-limit warning and exit with code 1.

3.3 Portability
NFR-3.1: The system MUST compile cleanly on Windows, Linux, and macOS using the standard cargo build --release toolchain.

NFR-3.2: The system MUST NOT require external dependencies, interpreters (Python/Node), or dynamic libraries beyond the standard OS network stack to execute.
