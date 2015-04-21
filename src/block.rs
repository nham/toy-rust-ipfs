use super::{MultiHash, HashFnCode};

// A singular block of data in IPFS
pub struct Block {
    mh: MultiHash,
    data: Vec<u8>,
}

impl Block {
    pub fn new() -> Block {
        Block {
            mh: MultiHash::new(),
            data: Vec::new(),
        }
    }

    pub fn data(data: Vec<u8>) -> Block {
        let code = HashFnCode::Sha1 as u8;
        Block {
            mh: match MultiHash::encode(data, code) {
                    Ok(mh) =>  mh,
                    Err(e) => panic!("Something is broken in Block::data"),
                },
            data: data,
        }
    }

    // This function trusts that the MultiHash `mh` is
    // correct for `data`
    //
    //  TODO: Have a debug mode where we go ahead and
    //  encode `data` to verify that `mh` is correct
    pub fn data_with_hash(data: Vec<u8>, mh: MultiHash) -> Block {
        Block {
            mh: mh,
            data: data,
        }
    }
}
