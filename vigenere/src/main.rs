use core::validate_key;

use clap::Parser;
use cli::{input, Args};
use constants::ALPHABET;
use crypt::{decrypt, encrypt};
use util::{get_mappings, print_alphabet};

mod cli;
mod constants;
mod core;
mod crypt;
mod util;

fn main() {
    let (char_to_idx, idx_to_char) = get_mappings(&ALPHABET);

    let args = Args::parse();

    println!("Vigenere's cypher. Alphabet:");
    print_alphabet(&ALPHABET);
    println!();

    match args.command {
        cli::Command::Encrypt => {
            let text = input("Enter your text");
            let key = input("Enter your key");
            if !validate_key(&key, &ALPHABET) {
                println!("The key is empty or contains forbidden characters");
                return;
            }
            let encrypted = encrypt(&text, &key, &ALPHABET, &char_to_idx, &idx_to_char);
            println!("Encrypted: {}", encrypted);
        }
        cli::Command::Decrypt => {
            let encrypted = input("Enter your encrypted text");
            let key = input("Enter your key");
            if !validate_key(&key, &ALPHABET) {
                println!("The key is empty or contains forbidden characters");
                return;
            }
            let text = decrypt(&encrypted, &key, &ALPHABET, &char_to_idx, &idx_to_char);
            println!("Decrypted: {}", text);
        }
    }
}
