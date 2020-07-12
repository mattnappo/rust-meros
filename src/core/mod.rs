pub trait Compressable {
    fn compress(&self) -> Vec<u8>;
    fn decompress(bytes: Vec<u8>) -> Self;
}
