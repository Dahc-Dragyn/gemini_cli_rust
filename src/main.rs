use gemini_cli_rust::cli::{Cli, Commands};
use gemini_cli_rust::error::{Error, Result};
use std::process;

use gemini_cli_rust::api::client::GeminiClient;

/// Asynchronous monolithic binary entry point.
/// Initializes the Tokio execution system and manages global error states without raw panic traces.
#[tokio::main]
async fn main() {
    // Execute application logic and catch all error boundaries cleanly
    dotenvy::dotenv().ok();
    if let Err(err) = run().await {
        match err {
            Error::Cli(ref cli_err) => {
                eprintln!("\x1b[1;31mCLI Routing Failure:\x1b[0m {}", cli_err);
                process::exit(2);
            }
            _ => {
                eprintln!("\x1b[1;31mExecution Error:\x1b[0m {}", err);
                process::exit(1);
            }
        }
    }
}

use gemini_cli_rust::io::{exec, pipe};

/// Dispatches parsed CLI commands to their respective asynchronous logic handlers.
async fn run() -> Result<()> {
    // Parse arguments from shell parameters
    let args = Cli::parse_args();

    match args.command {
        Commands::Ask(ask_args) => {
            // Validate temperature boundaries if provided
            if let Some(temp) = ask_args.temp {
                if !(0.0..=1.0).contains(&temp) {
                    return Err(Error::Cli(format!(
                        "Invalid temperature value: {}. Must be between 0.0 and 1.0.",
                        temp
                    )));
                }
            }

            // Ingest standard input context if a UNIX pipeline is active
            let piped_context = pipe::get_piped_context()?;

            // Concatenate standard input context and the CLI positional prompt
            let final_prompt = match piped_context {
                Some(ref context) => {
                    format!("Context:\n{}\n\nPrompt:\n{}", context, ask_args.prompt)
                }
                None => ask_args.prompt.clone(),
            };

            // Initialize connection client (fails if GEMINI_API_KEY is not defined)
            let client = GeminiClient::new()?;

            // Route execution based on requested real-time streaming flags
            if ask_args.stream {
                client
                    .ask_stream(&final_prompt, &ask_args.model, ask_args.temp)
                    .await?;
            } else {
                let generated_text = client
                    .ask(&final_prompt, &ask_args.model, ask_args.temp)
                    .await?;
                println!("{}", generated_text);
            }
        }
        Commands::Cmd(cmd_args) => {
            // Ingest standard input context if a UNIX pipeline is active
            let piped_context = pipe::get_piped_context()?;

            // Concatenate standard input context and the CLI positional prompt
            let raw_prompt = match piped_context {
                Some(ref context) => {
                    format!("Context:\n{}\n\nPrompt:\n{}", context, cmd_args.prompt)
                }
                None => cmd_args.prompt.clone(),
            };

            // Inject strict formatting system directive prompt
            let system_injection = format!(
                "You are a CLI command generator for Windows PowerShell and Unix. \
                 The user will ask for a task. Output ONLY the raw, executable terminal command. \
                 Do not provide explanations. Do not wrap the command in markdown formatting or backticks. \
                 Return the command and nothing else. Task: {}",
                raw_prompt
            );

            // Initialize connection client (fails if GEMINI_API_KEY is not defined)
            let client = GeminiClient::new()?;

            // Execute live one-shot generation using the default model
            let response = client
                .ask(&system_injection, "gemini-3.1-flash-lite", None)
                .await?;

            // Sanitize returned command
            let sanitized_command = sanitize_command(&response);

            println!("\nSuggested Command:\n{}\n", sanitized_command);

            // Ask user for safe subprocess execution confirmation
            if exec::confirm_execution(&sanitized_command)? {
                exec::execute_command(&sanitized_command)?;
            } else {
                println!("Execution cancelled.");
            }
        }
    }

    Ok(())
}

/// Sanitizes output commands by removing leading/trailing whitespaces or markdown delimiters.
fn sanitize_command(output: &str) -> String {
    let mut cleaned = output.trim().to_string();

    // Remove leading markdown blocks (e.g. ```powershell or ```bash)
    if cleaned.starts_with("```") {
        if let Some(first_newline) = cleaned.find('\n') {
            cleaned = cleaned[first_newline + 1..].to_string();
        }
    }

    // Remove trailing markdown block backticks
    if cleaned.ends_with("```") {
        cleaned = cleaned[..cleaned.len() - 3].to_string();
    }

    cleaned.trim().to_string()
}
