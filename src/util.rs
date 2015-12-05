use rust_multihash as multihash;

use std::fs::{self, File};
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

pub fn ensure_dir_writable<P: AsRef<Path>>(path: P) -> io::Result<()> {
    try!(fs::create_dir_all(&path));

    let mut path = path.as_ref().to_path_buf();
    path.push(".test_writable");
    try!(File::create(&path));
    fs::remove_file(path)
}
