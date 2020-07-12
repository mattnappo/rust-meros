use blake3;

/// The default size of hashes
const HASH_SIZE: usize = 32;

/// A type alias for Hashes to be used in the `primitives` module.
pub type Hash = [u8; HASH_SIZE];

pub fn hash_bytes(bytes: Vec<u8>) -> Hash {
    *blake3::hash(&bytes).as_bytes()
}
