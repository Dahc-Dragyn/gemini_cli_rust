use clap::{Parser, Subcommand};

/// Command-Line Interface representation for the `gemini_cli_rust` application.
/// Uses Clap's declarative derive model to enforce strict, zero-allocation schema parsing.
#[derive(Parser, Debug)]
#[command(
    name = "gemini-rs",
    author = "Antigravity Project Team",
    version = "1.0.0",
    about = "High-performance UNIX-idiomatic CLI wrapper for the Google Gemini API"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported subcommands for command routing.
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Transmits a query/prompt to the Gemini model and renders the output.
    Ask(AskArgs),
}

/// Input arguments and configuration flags for the `ask` subcommand.
#[derive(clap::Args, Debug, Clone)]
pub struct AskArgs {
    /// The primary instruction or query sent to the model.
    #[arg(required = true, help = "The primary instruction/query for the model")]
    pub prompt: String,

    /// Trigger real-time, chunked token streaming to stdout.
    #[arg(
        short = 's',
        long = "stream",
        help = "Enable real-time token rendering/streaming"
    )]
    pub stream: bool,

    /// Override the target model identifier.
    #[arg(
        short = 'm',
        long = "model",
        default_value = "gemini-3.1-flash-lite",
        help = "Target specific model variant"
    )]
    pub model: String,

    /// Set the model generation temperature (0.0 to 1.0).
    #[arg(
        short = 't',
        long = "temp",
        help = "Set the model generation temperature (0.0 to 1.0)"
    )]
    pub temp: Option<f32>,
}

impl Cli {
    /// Parses CLI arguments from system environment.
    /// Exits gracefully if syntax requirements are unmet.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
