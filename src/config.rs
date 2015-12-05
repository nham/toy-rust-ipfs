use crypto;
use util;

use rustc_serialize::base64::{self, ToBase64};
use rust_multihash::Multihash;
use std::path::PathBuf;

pub const DEFAULT_REPO_ROOT: &'static str = "~/";
pub const DEFAULT_REPO_PATH: &'static str = ".rust-ipfs";
pub const DEFAULT_CONFIG_FILE: &'static str = "config";
pub const ENV_NAME_REPO_DIR: &'static str = "IPFS_PATH";

pub const DEFAULT_KEYPAIR_NUM_BITS: usize = 2048;


pub struct Identity {
    pub peer_id: Multihash,
    pub private_key: String,
}

pub struct Config {
    pub id: Identity,
}

pub fn repo_path_to_config_file(mut repo_path: PathBuf) -> PathBuf {
    repo_path.push(DEFAULT_CONFIG_FILE);
    repo_path
}

pub fn init(num_key_pair_bits: usize) -> Config {
    let pkey = crypto::gen_key_pair(num_key_pair_bits);

    let pub_bytes = pkey.save_pub();
    let priv_b64_string = pkey.save_priv().to_base64(base64::STANDARD);
    Config {
        id: Identity {
            peer_id: util::hash(&pub_bytes[..]),
            private_key: priv_b64_string,
        }
    }
}
