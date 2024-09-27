use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
};

use derive_more::derive::Display;

use crate::constants::CHECKSUM_FILE_NAME;

pub(crate) type HashedFiles = HashMap<String, io::Result<u16>>;
pub(crate) type ParsedHashedFiles = HashMap<String, u16>;

/// Calculates hashes of files in the current working directory.
pub(crate) fn calculate_hashes() -> HashedFiles {
    let mut acc = HashMap::new();
    calculate_hashes_rec(&mut acc, Path::new("."));
    acc
}

fn calculate_hashes_rec(acc: &mut HashedFiles, path: &Path) {
    if path.ends_with(CHECKSUM_FILE_NAME) {
        return;
    }

    if path.is_dir() {
        // hash of the directory (always 0)
        acc.insert(path.display().to_string(), Ok(0));

        // hash of the contents
        match fs::read_dir(path) {
            Err(e) => println!("couldn't list contents of {}: {}", path.display(), e),
            Ok(listing) => listing.flatten().for_each(|entry| {
                let path = entry.path();
                calculate_hashes_rec(acc, &path);
            }),
        }
    } else if path.is_file() {
        // hash of the file
        acc.insert(path.display().to_string(), hash(path));
    }
}

/// Takes a path to the file and returns a hash.
fn hash(f: &Path) -> io::Result<u16> {
    let file = fs::File::open(f)?;
    let reader = io::BufReader::new(file);
    // calculate the checksum by streaming a file and XORing
    Ok(read_foldr(reader, |acc, curr| acc ^ curr)?)
}

/// Iterates over the contents of the `reader`, reading two bytes at a time,
/// and applying the "fold right" operation to them using `f` function.
///
/// When it converts bytes read to `u16`, it interprets them as little-endian.
///
/// The first argument to `f` is an accumulator, the second is the current
/// two bytes.
fn read_foldr(mut reader: impl io::Read, f: impl Fn(u16, u16) -> u16) -> io::Result<u16> {
    let mut acc = 0u16;
    loop {
        let mut buf = [0u8; 2];
        let count = reader.read(&mut buf)?;
        if count == 0 {
            // EOF
            break;
        }

        let curr = u16::from_le_bytes(buf);
        acc = f(acc, curr);
    }
    Ok(acc)
}

/// Writes tree's `hashes` to a given `output` file.
pub(crate) fn write_hashes(hashes: HashedFiles) -> io::Result<()> {
    let output = Path::new(".").join(CHECKSUM_FILE_NAME);
    let results_file = fs::File::create(output)?;
    let mut writer = io::BufWriter::new(results_file);
    for (k, v) in hashes.iter() {
        match v {
            Err(e) => println!("failed to calculate hash for {}: {}", k, e),
            Ok(checksum) => {
                let result = format!("{}: {}\n", k, checksum);
                writer.write_all(result.as_bytes())?
            }
        }
    }
    Ok(())
}

pub(crate) fn read_hashes_file() -> io::Result<ParsedHashedFiles> {
    let input = Path::new(".").join(CHECKSUM_FILE_NAME);
    let parsed = fs::read_to_string(input)?
        .lines()
        .into_iter()
        .filter_map(|line| line.rsplit_once(": "))
        .map(|(f, str_c)| (f, u16::from_str_radix(str_c, 10)))
        .filter_map(|(f, res)| res.ok().map(|c| (f.to_owned(), c)))
        .collect();

    Ok(parsed)
}

pub(crate) fn checksum_file_exists() -> bool {
    let checksum_path = Path::new(".").join(CHECKSUM_FILE_NAME);
    checksum_path.is_file()
}

#[derive(Display, Debug)]
pub(crate) enum ReportEntry {
    #[display("unmodified(now = {now})")]
    Unmodified { now: u16 },
    #[display("modified(was = {was}, now = {now})")]
    Modified { was: u16, now: u16 },
    #[display("deleted")]
    Deleted { was: u16 },
    #[display("new")]
    New,
}

pub(crate) type CheckReport = HashMap<String, io::Result<ReportEntry>>;

/// Compares previous and freshly calculated hashes.
pub(crate) fn compare_hashes(mut previous: ParsedHashedFiles, current: HashedFiles) -> CheckReport {
    let mut report: CheckReport = current
        .into_iter()
        .map(|(f, c)| match previous.remove(&f) {
            None => (f, Ok(ReportEntry::New)),
            Some(p) => (
                f,
                c.map(|c| {
                    if p == c {
                        ReportEntry::Unmodified { now: c }
                    } else {
                        ReportEntry::Modified { was: p, now: c }
                    }
                }),
            ),
        })
        .collect();

    previous.into_iter().for_each(|(f, p)| {
        report.insert(f, Ok(ReportEntry::Deleted { was: p }));
    });

    report
}

/// Prints the difference `report` to the stdout.
pub(crate) fn print_diff(report: CheckReport) {
    report
        .into_iter()
        .filter_map(|(f, r)| {
            r.map_err(|e| {
                println!("failed to calculate hash for {}: {}", &f, e);
                e
            })
            .map(|v| (f, v))
            .ok()
        })
        .for_each(|(f, r)| match r {
            ReportEntry::Unmodified { .. } => {}
            _ => println!("{}: {}", f, r),
        });
}
