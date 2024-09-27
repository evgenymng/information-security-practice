use std::ffi::OsString;

use clap::value_parser;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    about = "Steganography utility",
    long_about = None,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Hide {
        /// A container file to use as base to hide the text in.
        file: String,
        /// A text to hide.
        #[arg(value_parser(value_parser!(OsString)))]
        msg: OsString,
        /// A file to output text to (nothing for STDOUT).
        #[arg(short, long)]
        output: Option<String>,
    },
    Unhide {
        /// A container file with a hidden text.
        file: String,
        /// A file to output text to (nothing for STDOUT).
        #[arg(short, long)]
        output: Option<String>,
    },
}
