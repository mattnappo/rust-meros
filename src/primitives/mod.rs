pub mod file;

/// The default size of hashes
const HASH_SIZE: u8; 32;

/// A type alias for Hashes to be used in the `primitives` module.
type Hash = [u8; HASH_SIZE];

/// A trait given to types that are able to be hashed.
trait Hashable {
    fn hash(&self) -> Vec<u8>;
}
