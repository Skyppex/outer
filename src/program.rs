use std::process::Command;

use crate::args::OuterArgs;

#[derive(Debug)]
pub struct Output {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl From<std::process::Output> for Output {
    fn from(output: std::process::Output) -> Self {
        Output {
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

pub fn run(args: OuterArgs) -> Result<Output, String> {
    let verbose = args.verbose;
    if verbose {
        eprintln!("Running command: '{}'", args.command.join(" "));
    }

    let args = args.command;

    let cmd = args.get(0).ok_or("No command provided")?;

    if verbose {
        eprintln!("Command: '{}'", cmd);
        eprintln!(
            "Args: '{:?}'",
            args.clone().into_iter().skip(1).collect::<Vec<String>>()
        );
    }

    let output = Command::new(cmd)
        .stdin(std::process::Stdio::piped())
        .args(args.iter().skip(1))
        .output()
        .map_err(|e| format!("Failed to run command '{}' | Error: {}", args.join(" "), e,))?;

    if verbose {
        eprintln!("Output: '{:?}'", output);
    }

    Ok(output.into())
}
#[cfg(test)]
mod tests {}
