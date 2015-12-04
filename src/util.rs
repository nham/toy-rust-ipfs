use rust_multihash as multihash;

use std::fs;
use std::io;
use std::path::Path;

pub fn hash<'a>(data: &'a[u8]) -> multihash::Multihash {
    multihash::multihash(data, multihash::HashType::SHA2_256)
}

// Expects that there will not be permissions errors
// panics if there is.
pub fn file_exists_expect<P: AsRef<Path>>(path: P) -> bool {
    file_exists(path).expect("Error calling file_exists()")
}

pub fn file_exists<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    match fs::metadata(path) {
        Err(e) =>
            match e.kind() {
                io::ErrorKind::NotFound => Ok(false),
                _ => Err(e),
            },
        Ok(_) => Ok(true),
    }
}
