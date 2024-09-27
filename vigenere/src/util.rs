use std::collections::HashMap;

/// Returns char -> index and index -> char mappings
/// for a given alphabet.
pub(crate) fn get_mappings(alpha: &[char]) -> (HashMap<char, usize>, HashMap<usize, char>) {
    (
        alpha.iter().enumerate().map(|(i, c)| (*c, i)).collect(),
        alpha.iter().enumerate().map(|(i, c)| (i, *c)).collect(),
    )
}

pub(crate) fn print_alphabet(alpha: &[char]) {
    alpha.iter().for_each(|c| print!("{}", c))
}
