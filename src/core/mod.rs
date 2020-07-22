use crate::primitives::shard::Shard;

/// The errors that could be thrown by the `core` module.
pub enum CoreError {}

/// Add compression and decompression functionality to a serializable type.
pub trait Compressable {
    fn compress(&self) -> Vec<u8>;
    fn decompress(bytes: Vec<u8>) -> Self;
}
