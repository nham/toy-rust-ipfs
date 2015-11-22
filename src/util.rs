use multihash;

pub fn hash<'a>(data: &'a[u8]) -> multihash::Multihash {
    multihash::multihash(data, multihash::HashType::SHA2_256)
}
