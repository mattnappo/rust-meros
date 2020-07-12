pub mod file;
pub mod shard;

/// A trait given to types that are able to be hashed.
trait Hashable {
    fn hash(&self) -> crate::crypto::hash::Hash;
}
