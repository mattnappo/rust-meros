pub trait CanEncrypt {
    fn encrypt(&self) -> Vec<u8>;
    fn decrypt(bytes: Vec<u8>) -> Self;
}
