use std::{
    collections::HashMap,
    io::{self, Write},
};

use bit_vec::BitVec;
use clap::Parser;
use cli::{Args, Command};
use util::{read_file, replacement, write_file};

mod cli;
mod util;

/// Hides `msg` bit by bit within the `cont`.
fn hide(ru_to_en: &HashMap<u8, u8>, cont: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    let mut msg = BitVec::from_bytes(msg);
    msg.append(&mut BitVec::from_elem(8, false)); // append \0

    for bit in msg.iter() {
        while res.len() < cont.len() && replacement(ru_to_en, cont[res.len()]).is_none() {
            res.push(cont[res.len()]);
        }
        if bit {
            let rb = match replacement(ru_to_en, cont[res.len()]) {
                Some(rb) => rb,
                None => return res, // reached the end of `cont`
            };
            res.push(rb);
        } else {
            res.push(cont[res.len()]);
        }
    }

    let mut rest = Vec::from(&cont[res.len()..cont.len()]);
    res.append(&mut rest);
    res
}

/// Extracts the message bit by bit from the `text`.
fn unhide(ru_to_en: &HashMap<u8, u8>, en_to_ru: &HashMap<u8, u8>, text: &[u8]) -> Vec<u8> {
    let mut res = BitVec::new();
    let mut bit_index = 0;
    for byte in text.iter() {
        if replacement(ru_to_en, *byte).is_some() {
            res.push(false)
        } else if replacement(en_to_ru, *byte).is_some() {
            res.push(true)
        } else {
            continue;
        }

        bit_index = (bit_index + 1) % 8;
        if bit_index == 0 {
            // byte is done
            let res_bytes = res.to_bytes(); // inefficient, but I am tired
            if res_bytes[res_bytes.len() - 1] == 0 {
                return res_bytes;
            }
        }
    }
    // text has ended
    res.to_bytes()
}

fn main() {
    let ru_to_en = HashMap::from([
        (0xe0, 0x61), // a
        (0xe5, 0x65), // e
        (0xee, 0x6f), // o
        (0xf0, 0x70), // p
        (0xf1, 0x63), // c
        (0xf3, 0x79), // y
        (0xf5, 0x78), // x
        (0xc0, 0x41), // A
        (0xc2, 0x42), // B
        (0xc5, 0x45), // E
        (0xca, 0x4b), // K
        (0xce, 0x4f), // O
        (0xd0, 0x50), // P
        (0xd1, 0x43), // C
        (0xd2, 0x54), // T
        (0xd5, 0x58), // X
    ]);

    let en_to_ru = HashMap::from([
        (0x61, 0xe0), // a
        (0x65, 0xe5), // e
        (0x6f, 0xee), // o
        (0x70, 0xf0), // p
        (0x63, 0xf1), // c
        (0x79, 0xf3), // y
        (0x78, 0xf5), // x
        (0x41, 0xc0), // A
        (0x42, 0xc2), // B
        (0x45, 0xc5), // E
        (0x4b, 0xca), // K
        (0x4f, 0xce), // O
        (0x50, 0xd0), // P
        (0x43, 0xd1), // C
        (0x54, 0xd2), // T
        (0x58, 0xd5), // X
    ]);

    let args = Args::parse();
    match &args.command {
        Command::Hide { file, msg, output } => {
            let cont = match read_file(file) {
                Err(e) => {
                    println!("failed to read container file {}: {}", file, e);
                    return;
                }
                Ok(s) => s,
            };

            let msg = msg.as_encoded_bytes();
            println!("(debug) hiding message: {}", BitVec::from_bytes(msg));
            let hidden = hide(&ru_to_en, &cont, msg);

            let res = match output {
                Some(o) => write_file(o, hidden),
                None => io::stdout().write(&hidden).map(|_| ()),
            };
            if let Err(e) = res {
                println!("failed to write result: {}", e);
            }
        }
        Command::Unhide { file, output } => {
            let hidden = match read_file(file) {
                Err(e) => {
                    println!("failed to read file {}: {}", file, e);
                    return;
                }
                Ok(s) => s,
            };

            let extracted = unhide(&ru_to_en, &en_to_ru, &hidden);
            println!("(debug) extracted: {}", BitVec::from_bytes(&extracted));

            let res = match output {
                Some(o) => write_file(o, extracted),
                None => io::stdout().write(&extracted).map(|_| ()),
            };
            if let Err(e) = res {
                println!("failed to write result: {}", e);
            }
        }
    }
}
