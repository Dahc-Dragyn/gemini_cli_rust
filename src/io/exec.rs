use crate::error::{Error, Result};
use std::io::{stdout, Write};
use std::process::Command;

/// Prompts the user for execution confirmation via stdin.
/// Returns `Ok(true)` if input matches 'y' or 'Y', otherwise `Ok(false)`.
pub fn confirm_execution(_command: &str) -> Result<bool> {
    print!("[?] Would you like to execute this command? (y/N): ");
    stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let trimmed = input.trim().to_lowercase();
    Ok(trimmed == "y" || trimmed == "yes")
}

/// Executes a system command natively based on OS detection.
/// Windows targets run powershell, Unix targets run sh.
pub fn execute_command(command: &str) -> Result<()> {
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("powershell");
        c.args(["-Command", command]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", command]);
        c
    };

    // Inherit standard output and error handles so outputs render natively
    let mut child = cmd
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        return Err(Error::Cli(format!(
            "Command execution exited with non-zero status: {}",
            status
        )));
    }

    Ok(())
}
