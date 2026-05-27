use thiserror::Error;

/// Core error representation enum for all fallible operations in `gemini_cli_rust`.
/// Strictly adheres to the Zero-Panic directive.
#[derive(Debug, Error)]
pub enum Error {
    #[error("API Key Missing: Please set the GEMINI_API_KEY environment variable.")]
    MissingApiKey,

    #[error("Network Error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Rate Limit Exceeded (HTTP 429): Too many requests. Please apply backoff and try again later.")]
    RateLimit,

    #[error("Authentication Failed (HTTP 401/403): Invalid GEMINI_API_KEY provided.")]
    Unauthorized,

    #[error("API Error (HTTP {status}): {message}")]
    ApiError {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("UNIX Standard Input Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CLI Argument Error: {0}")]
    Cli(String),
}

/// Type alias for Results returned by `gemini_cli_rust` domain logic.
pub type Result<T> = std::result::Result<T, Error>;
