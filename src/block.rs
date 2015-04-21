use super::MultiHash;

pub struct Block {
    mh: MultiHash,
    data: Vec<u8>,
}
