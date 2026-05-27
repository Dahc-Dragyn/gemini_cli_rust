use crate::error::Result;
use std::io::{stdin, IsTerminal, Read};

/// Detects if data is piped via standard input (stdin) and ingests it.
/// Returns `Ok(Some(data))` if a pipe is active and data is read successfully.
/// Returns `Ok(None)` if running in an interactive terminal (TTY) environment.
pub fn get_piped_context() -> Result<Option<String>> {
    // If standard input is not a terminal, we are receiving piped context
    if !stdin().is_terminal() {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        Ok(Some(buffer))
    } else {
        Ok(None)
    }
}
