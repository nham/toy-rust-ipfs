use block::Block;
use util;

use atomicwrites::{AtomicFile, DisallowOverwrite};
use rust_multihash::Multihash;
use rustc_serialize::hex::ToHex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};

const BLOCKFILE_EXT: &'static str = ".data";
const BLOCKSTORE_PREFIX_LENGTH: u8 = 4; // in bytes
pub const BLOCKSTORE_DIR: &'static str = "blocks";

pub struct Blockstore {
    path: PathBuf,
    hex_prefix_length: u8, // length of the prefix in hex digits
}

impl Blockstore {
    // When a block with multihash bytes of [x1 ... xn] is put to the Blockstore,
    // the block data will be stored as a file in the directory
    // <blockstore dir>/<hex encoding of [x1 ... xk]/
    // where k = prefix_len
    pub fn new(path: PathBuf) -> Self {
        Blockstore {
            path: path,
            hex_prefix_length: 2 * BLOCKSTORE_PREFIX_LENGTH,
        }
    }

    pub fn has(&self, multihash: &Multihash) -> Result<bool, String> {
        util::file_exists(self.block_file(multihash)).map_err(|e| {
            format!("Error checking for existence of file in Blockstore::has: {}",
                    e)
        })
    }

    pub fn get(&self, hash: &Multihash) -> Result<Block, String> {
        let fname = self.block_file(hash);
        trace!("Blockstore::get, file name = {:?}", fname);
        let mut file = try!(File::open(&fname).map_err(|e| {
            format!("Error opening file {:?} for hash {} in Blockstore::get: {}",
                    fname,
                    hash,
                    e)
        }));

        let mut data = Vec::new();
        try!(file.read_to_end(&mut data)
                 .map_err(|e| {
                     format!("Error reading file {:?} for hash {:?} in Blockstore::get: {}",
                             fname,
                             hash,
                             e)
                 }));
        Ok(Block::with_hash(data, hash.clone()))
    }

    pub fn put(&self, multihash: &Multihash, data: &[u8]) -> Result<(), String> {
        match self.has(multihash) {
            Ok(true) => return Ok(()),
            _ => {}
        }

        let (mut dir, filename) = self.block_dir_and_file(multihash);
        try!(make_prefix_dir(&dir).map_err(|e| {
            format!("Error making prefix directory for put: {}", e)
        }));


        dir.push(filename);
        let file_path = dir; // rename for clarity
        let file = AtomicFile::new(file_path, DisallowOverwrite);
        file.write(|f| f.write_all(data))
            .map_err(|e| format!("Error writing block file for put: {}", e))
    }

    fn block_dir_and_file(&self, multihash: &Multihash) -> (PathBuf, String) {
        let hex = multihash.to_hex();
        let mut dir = self.path.clone();
        dir.push(&hex[..self.hex_prefix_length as usize]);

        (dir, hex + BLOCKFILE_EXT)
    }

    fn block_file(&self, multihash: &Multihash) -> PathBuf {
        let (mut dir, filename) = self.block_dir_and_file(multihash);
        dir.push(filename);
        dir
    }
}


fn make_prefix_dir<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    try!(fs::create_dir(&path));

    // TODO: is this even needed? ensure
    let f = try!(fs::File::open(&path));
    f.sync_all()
}
