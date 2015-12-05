use openssl::crypto::pkey;

pub fn gen_key_pair(num_bits: usize) -> pkey::PKey {
    let mut key_pair = pkey::PKey::new();
    key_pair.gen(num_bits);
    key_pair
}
