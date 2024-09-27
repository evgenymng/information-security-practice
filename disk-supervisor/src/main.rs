mod cli;
mod constants;
mod tree;

use std::{error::Error, path::Path};

use clap::Parser;
use constants::CHECKSUM_FILE_NAME;
use tree::{
    calculate_hashes, checksum_file_exists, compare_hashes, print_diff, read_hashes_file,
    write_hashes,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    match &args.command {
        cli::Command::Generate { path } | cli::Command::Check { path } => {
            // while not great, this is to bypass lifetime restrictions
            let curr_dir = &std::env::current_dir()?;
            let workdir = match path {
                Some(p) => Path::new(p),
                None => curr_dir,
            };
            if let Err(e) = std::env::set_current_dir(workdir) {
                println!("failed to cd into {}: {}", workdir.display(), e);
                return Ok(());
            }
        }
    }

    match args.command {
        cli::Command::Generate { .. } => {
            let hashes = calculate_hashes();
            write_hashes(hashes)?;
        }
        cli::Command::Check { .. } => {
            if !checksum_file_exists() {
                println!("checksum file ({}) doesn't exist", CHECKSUM_FILE_NAME);
                return Ok(());
            }

            let previous = match read_hashes_file() {
                Err(e) => {
                    println!(
                        "failed to read the checksum file {}: {}",
                        CHECKSUM_FILE_NAME, e,
                    );
                    return Ok(());
                }
                Ok(hashes) => hashes,
            };

            let current = calculate_hashes();
            let report = compare_hashes(previous, current);
            print_diff(report);
        }
    }

    Ok(())
}
