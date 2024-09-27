use std::io::{self, Write};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    about = "Vigenere cypher utility",
    long_about = None,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Encrypt the text using key.
    Encrypt,
    /// Decrypt the text using key.
    Decrypt,
}

pub(crate) fn input(prompt: &str) -> String {
    print!("{}\n> ", prompt);
    let _ = io::stdout().flush();
    let mut buf = String::new();
    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(_) => break,
            Err(e) => {
                print!("failed to read: {}\n> ", e);
                let _ = io::stdout().flush();
            }
        }
    }
    buf.trim().to_owned()
}
