use crypto;
use util;

use rustc_serialize::Decodable;
use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::json::{self, Json};
use rust_multihash::Multihash;
use std::io::Read;
use std::path::PathBuf;

pub const DEFAULT_REPO_ROOT: &'static str = "~/";
pub const DEFAULT_REPO_PATH: &'static str = ".rust-ipfs";
pub const DEFAULT_CONFIG_FILE: &'static str = "config";
pub const ENV_NAME_REPO_DIR: &'static str = "IPFS_PATH";

pub const DEFAULT_KEYPAIR_NUM_BITS: usize = 2048;


#[derive(RustcEncodable, RustcDecodable)]
pub struct Identity {
    pub peer_id: Multihash,
    pub private_key: String,
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
    pub identity: Identity,
}

impl Config {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Config, String> {
        let json = try!(Json::from_reader(reader)
                        .map_err(|e| format!("Error parsing Json: {}", e)));
        let mut decoder = json::Decoder::new(json);
        Decodable::decode(&mut decoder)
            .map_err(|e| format!("Error decoding Config from reader: {}", e))
    }

    pub fn to_json_string(&self) -> json::EncodeResult<String> {
        json::encode(self)
    }
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
        identity: Identity {
            peer_id: util::hash(&pub_bytes[..]),
            private_key: priv_b64_string,
        }
    }
}
