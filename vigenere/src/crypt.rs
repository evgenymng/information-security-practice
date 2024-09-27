use std::collections::HashMap;

use crate::core::LoopingKey;

/// Encrypts the `text` using the Vigenere's cypher.
///
/// The key must not contain any characters not from the alphabet
/// and must not be empty.
///
/// - `t_char`: initial text character.
/// - `k_char`: character from the key.
pub(crate) fn encrypt(
    text: &str,
    key: &str,
    alphabet: &[char],
    char_to_idx: &HashMap<char, usize>,
    idx_to_char: &HashMap<usize, char>,
) -> String {
    let alpha_len = alphabet.len();
    text.chars()
        .zip(LoopingKey::new(key))
        .map(|(t_char, k_char)| {
            let k_idx = char_to_idx.get(&k_char).unwrap();
            match char_to_idx.get(&t_char) {
                None => t_char,
                Some(t_idx) => {
                    let e_idx = (t_idx + k_idx) % alpha_len;
                    *idx_to_char.get(&e_idx).unwrap()
                }
            }
        })
        .collect()
}

/// Decrypts the `text` using the Vigenere's cypher.
///
/// The key must not contain any characters not from the alphabet
/// and must not be empty.
///
/// - `e_char`: character from the encrypted text.
/// - `k_char`: character from the key.
pub(crate) fn decrypt(
    encrypted: &str,
    key: &str,
    alphabet: &[char],
    char_to_idx: &HashMap<char, usize>,
    idx_to_char: &HashMap<usize, char>,
) -> String {
    let alpha_len = alphabet.len();
    encrypted
        .chars()
        .zip(LoopingKey::new(key))
        .map(|(e_char, k_char)| {
            let k_idx = char_to_idx.get(&k_char).unwrap();
            match char_to_idx.get(&e_char) {
                None => e_char,
                Some(e_idx) => {
                    let t_idx = (alpha_len + e_idx - k_idx) % alpha_len;
                    *idx_to_char.get(&t_idx).unwrap()
                }
            }
        })
        .collect()
}
