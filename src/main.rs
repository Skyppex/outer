mod args;
mod program;

use std::{
    io::{self, Result, Write},
    process,
};

use args::OuterArgs;

fn main() -> Result<()> {
    let args = OuterArgs::parse();

    match program::run(args) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(output) => {
            match io::stdout().write_all(&output.stderr) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to stdout");
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }

            match io::stderr().write_all(&output.stdout) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to stderr");
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }

            Ok(())
        }
    }
}
