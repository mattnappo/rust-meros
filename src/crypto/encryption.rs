use crate::CanSerialize;

pub trait CanEncrypt<T>
where
    T: CanSerialize,
{
    fn encrypt(&self) -> Result<Vec<u8>, std::error::Error> {
        let bytes = <T as CanSerialize>::to_bytes()?;
    }
    fn decrypt(bytes: Vec<u8>) -> Self;
}
