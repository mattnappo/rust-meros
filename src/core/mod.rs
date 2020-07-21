use crate::primitives::shard::Shard;

/// The errors that could be thrown by the `core` module.
pub enum CoreError {}

/// Add compression and decompression functionality to a serializable type.
pub trait Compressable {
    fn compress(&self) -> Vec<u8>;
    fn decompress(bytes: Vec<u8>) -> Self;
}

pub fn split_bytes(bytes: Vec<u8>, sizes: Vec<u8>) -> Vec<Shard> {
    // this is a temporary implementation
    vec![Shard::new().map]
}
