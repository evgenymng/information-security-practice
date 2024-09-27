use std::{collections::HashMap, fs, io, path::Path};

/// Reads contents of the file.
pub(crate) fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let path = Path::new(filename);
    fs::read(path)
}

/// Writes bytes to the output file.
pub(crate) fn write_file(filename: &str, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let path = Path::new(filename);
    fs::write(path, bytes)
}

/// Finds replacement for the byte in the maps.
pub(crate) fn replacement(repl: &HashMap<u8, u8>, byte: u8) -> Option<u8> {
    repl.get(&byte).copied()
}
