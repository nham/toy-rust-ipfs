use self::HashType::*;

use openssl::crypto::hash as openssl_hash;

// https://github.com/jbenet/multihash
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HashType {
    SHA1 = 0x11,
    SHA2_256 = 0x12,
    SHA2_512 = 0x13,
    SHA3 = 0x14,
    BLAKE2B = 0x40,
    BLAKE2S = 0x41,
}

impl HashType {
    pub fn from_u8(x: u8) -> Option<HashType> {
        match x {
            0x11 => Some(SHA1),
            0x12 => Some(SHA2_256),
            0x13 => Some(SHA2_512),
            0x14 => Some(SHA3),
            0x40 => Some(BLAKE2B),
            0x41 => Some(BLAKE2S),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

pub fn multihash<'a>(data: &'a[u8], hash_type: HashType) -> Multihash {
    let mut openssl_type: openssl_hash::Type = match hash_type {
        SHA1 => openssl_hash::Type::SHA1,
        SHA2_256 => openssl_hash::Type::SHA256,
        SHA2_512 => openssl_hash::Type::SHA512,
        _ => panic!("That hash function is not yet implemented. Sorry"),
    };

    let mut hashed = openssl_hash::hash(openssl_type, data);

    Multihash::encode(&hashed[..], hash_type).unwrap()
}


pub struct Multihash(Vec<u8>);

impl Multihash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    // Constructs a Multihash out of the bytes of a hash and the
    // HashFnCode corresponding to the type of hash function used
    pub fn encode<'a>(digest: &'a [u8], hash_type: HashType) -> Result<Multihash, String> {
        let size = digest.len();
        if size > 127 {
            return Err("Digest length > 127 is currently not supported.".to_string())
        }

        let mut v = Vec::with_capacity(size + 2);
        v.push(hash_type.to_u8());
        v.push(size as u8);
        v.extend(digest);
        Ok(Multihash(v))
    }
}



#[cfg(test)]
mod tests {
    use super::{multihash, Multihash, HashType};
    use rustc_serialize::hex::FromHex;

    struct TestCase {
        hexstr: &'static str,
        hash_type: HashType,
    }

    impl TestCase {
        fn new(s: &'static str, ty: HashType) -> TestCase {
            TestCase { hexstr: s, hash_type: ty }
        }
    }

    const SHA1_STR: &'static str = "a228821137dacdbcd3ba5fa264f918fd
                                    a6223f7b";

    const SHA256_STR: &'static str = "95071c8e1ad3c7a016b30fd25853c1d9\
                                      e346ee7ee4afc977f554b139f71f4f30";

    const SHA512_STR: &'static str = "3912d4c1777934091ed95a15278b351e\
                                      00efa51f524af9af3320f0f6a227d551\
                                      76b1399684d15a05f03d1e519a9a9aa5\
                                      2a812a6d9bc63e5b485d6fd7ebf72114";

    #[test]
    fn test_multihash() {
        let abc = b"ABC";
        let sha1_hex = "3c01bdbb26f358bab27f267924aa2c9a03fcfdb8";
        let sha256_hex = "b5d4045c3f466fa91fe2cc6abe79232a1a57cdf104f7a26e716e0a1e2789df78";
        let sha512_hex = "397118fdac8d83ad98813c50759c85b8c47565d8268bf10da483153b747a7474\
                          3a58a90e85aa9f705ce6984ffc128db567489817e4092d050d8a1cc596ddc119";

        let sha1 = multihash(abc, HashType::SHA1);
        let mut sha1_bytes = hexstr_to_vec(sha1_hex);
        let mut expected_bytes = vec![0x11, 20];
        expected_bytes.append(&mut sha1_bytes);
        assert_eq!(sha1.as_bytes(), &expected_bytes[..]);

        let sha256 = multihash(abc, HashType::SHA2_256);
        let mut sha256_bytes = hexstr_to_vec(sha256_hex);
        let mut expected_bytes = vec![0x12, 32];
        expected_bytes.append(&mut sha256_bytes);
        assert_eq!(sha256.as_bytes(), &expected_bytes[..]);


        let sha512 = multihash(abc, HashType::SHA2_512);
        let mut sha512_bytes = hexstr_to_vec(sha512_hex);
        let mut expected_bytes = vec![0x13, 64];
        expected_bytes.append(&mut sha512_bytes);
        assert_eq!(sha512.as_bytes(), &expected_bytes[..]);
    }

    fn hexstr_to_vec(hexstr: &'static str) -> Vec<u8> {
        match hexstr.from_hex() {
            Err(e) => panic!("from_hex() failed with error: {:?}", e),
            Ok(vec) => vec,
        }
    }

    #[test]
    fn test_encode() {
        let mut cases = Vec::new();

        cases.push(TestCase::new(SHA1_STR, HashType::SHA1));
        cases.push(TestCase::new(SHA256_STR, HashType::SHA2_256));
        cases.push(TestCase::new(SHA512_STR, HashType::SHA2_512));

        for case in cases {
            let v = hexstr_to_vec(case.hexstr);

            let mut expected = Vec::new();
            expected.push(case.hash_type.to_u8());
            expected.push(v.len() as u8);
            expected.extend(&v[..]);

            let mh = Multihash::encode(&v[..], case.hash_type).unwrap();
            assert_eq!(&expected[..], mh.as_bytes());
        }
    }
}
