use super::CryptoError;
use crate::{
    primitives::{file::File, shard::Shard},
    CanSerialize,
};
use ecies_ed25519::{decrypt, encrypt, generate_keypair, PublicKey, SecretKey};
use rand;
use std::{
    fs::{create_dir_all, File as StdFile},
    io::{Read, Write},
    path::Path,
};

// A trait constrait for types that are keys
// (ed25519 public and private key only)
trait IsKey {}

impl IsKey for PublicKey {}
impl IsKey for SecretKey {}

const KEY_LOCATION: &str = "./data/keys/";

/// Specify the type of key and the name of the key.
pub enum KeyType {
    Public(String),
    Private(String),
}

/// A handy shorthand type representing a keypair.
type Keypair = (SecretKey, PublicKey);

/// Write a single key to the disk.
fn write_key<K>(key: &K, key_type: KeyType) -> Result<(), CryptoError>
where
    K: IsKey + serde::Serialize,
{
    let key =
        bincode::serialize(key).map_err(|e| CryptoError::SerializationError(e))?;

    let (name, extension) = match key_type {
        KeyType::Private(name) => (name, "priv"),
        KeyType::Public(name) => (name, "pub"),
    };

    let mut file =
        StdFile::create(format!("{}{}.{}", KEY_LOCATION, name, extension,).as_str())
            .map_err(|e| CryptoError::IOError(e))?;

    file.write_all(&key[..])
        .map_err(|e| CryptoError::IOError(e))?;
    Ok(())
}

/// Write a keypair to the disk. keys[0] = priv, keys[1] = pub.
fn write_keypair(
    pair: (&SecretKey, &PublicKey),
    name: &str,
) -> Result<(), CryptoError> {
    create_dir_all(Path::new(KEY_LOCATION)).map_err(|e| CryptoError::IOError(e))?;

    write_key(pair.0, KeyType::Private(name.to_string()))?;
    write_key(pair.1, KeyType::Public(name.to_string()))?;
    Ok(())
}

/// Generate a public-private keypair and write to disk with the given name.
pub fn gen_keypair(name: &str, write: bool) -> Result<Keypair, CryptoError> {
    let mut csprng = rand::thread_rng();
    let (priv_key, pub_key) = generate_keypair(&mut csprng);
    if write {
        write_keypair((&priv_key, &pub_key), name)?;
    }
    Ok((priv_key, pub_key))
}

/// Load a public key from the disk given the key name and type.
pub fn load_pub_key(key_type: &KeyType) -> Result<PublicKey, CryptoError> {
    // Get key path
    let loc = match key_type {
        KeyType::Public(name) => format!("{}{}.pub", KEY_LOCATION, name),
        KeyType::Private(name) => {
            return Err(CryptoError::InvalidKey(crate::GeneralError::new(
                format!("cannot load public key: {} is a private key", name,)
                    .as_str(),
            )))
        }
    };

    // Read the key as bytes
    let mut file =
        StdFile::open(loc.as_str()).map_err(|e| CryptoError::IOError(e))?;
    let mut key_buf = Vec::new();
    file.read_to_end(&mut key_buf)
        .map_err(|e| CryptoError::IOError(e))?;

    // Deserialize the key (todo: use a trait here)
    bincode::deserialize(&key_buf[..])
        .map_err(|e| CryptoError::SerializationError(e))
}

/// Load a private key from the disk given the key name and type.
pub fn load_priv_key(key_type: &KeyType) -> Result<SecretKey, CryptoError> {
    // Get key path
    let loc = match key_type {
        KeyType::Private(name) => format!("{}{}.priv", KEY_LOCATION, name),
        KeyType::Public(name) => {
            return Err(CryptoError::InvalidKey(crate::GeneralError::new(
                format!("cannot load private key: {} is a public key", name,)
                    .as_str(),
            )))
        }
    };

    // Read the key as bytes
    let mut file =
        StdFile::open(loc.as_str()).map_err(|e| CryptoError::IOError(e))?;
    let mut key_buf = Vec::new();
    file.read_to_end(&mut key_buf)
        .map_err(|e| CryptoError::IOError(e))?;

    // Deserialize the key (todo: use a trait here)
    bincode::deserialize(&key_buf[..])
        .map_err(|e| CryptoError::SerializationError(e))
}

fn load_keypair(name: &str) -> Result<Keypair, CryptoError> {
    Ok((
        load_priv_key(&KeyType::Private(name.to_string()))?,
        load_pub_key(&KeyType::Public(name.to_string()))?,
    ))
}

pub fn encrypt_bytes(
    key: &PublicKey,
    bytes: &Vec<u8>,
) -> Result<Vec<u8>, CryptoError> {
    let mut csprng = rand::thread_rng();
    encrypt(key, &bytes, &mut csprng).map_err(|e| CryptoError::EncryptionError(e))
}

pub fn decrypt_bytes(
    key: &SecretKey,
    bytes: &Vec<u8>,
) -> Result<Vec<u8>, CryptoError> {
    decrypt(key, &bytes).map_err(|e| CryptoError::EncryptionError(e))
}

pub trait CanEncrypt: CanSerialize {
    type D: CanEncrypt;

    fn encrypt(&self, key: PublicKey) -> Result<Vec<u8>, CryptoError>;

    fn decrypt(bytes: Vec<u8>, key: SecretKey) -> Result<Self::D, CryptoError>;
}

impl CanEncrypt for File {
    type D = Self;

    fn encrypt(&self, key: PublicKey) -> Result<Vec<u8>, CryptoError> {
        let bytes = self
            .to_bytes()
            .map_err(|e| CryptoError::SerializationError(e))?;
        encrypt_bytes(&key, &bytes)
    }

    fn decrypt(bytes: Vec<u8>, key: SecretKey) -> Result<Self::D, CryptoError> {
        let decrypted = decrypt_bytes(&key, &bytes)?;

        <Self::D as CanSerialize>::from_bytes(decrypted)
            .map_err(|e| CryptoError::SerializationError(e))
    }
}

impl CanEncrypt for Shard {
    type D = Self;

    fn encrypt(&self, key: PublicKey) -> Result<Vec<u8>, CryptoError> {
        let bytes = self
            .to_bytes()
            .map_err(|e| CryptoError::SerializationError(e))?;
        encrypt_bytes(&key, &bytes)
    }

    fn decrypt(bytes: Vec<u8>, key: SecretKey) -> Result<Self::D, CryptoError> {
        let decrypted = decrypt_bytes(&key, &bytes)?;

        <Self::D as CanSerialize>::from_bytes(decrypted)
            .map_err(|e| CryptoError::SerializationError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{file::File, shard::Shard};
    use crate::CanSerialize;
    use std::path::Path;

    #[test]
    fn test_encrypt_file() {
        let (file, _) = File::new(Path::new("testfile.txt"), None).unwrap();
        let keypair = gen_keypair("testkey", false).unwrap();
        file.encrypt(keypair.1).unwrap();
    }

    #[test]
    fn test_decrypt_file() {
        let (file, _) = File::new(Path::new("testfile.txt"), None).unwrap();
        let keypair = gen_keypair("testkey", false).unwrap();
        let encrypted = file.encrypt(keypair.1).unwrap();
        let decrypted = File::decrypt(encrypted, keypair.0).unwrap();
        assert_eq!(file, decrypted);
    }

    #[test]
    fn test_encrypt_shard() {
        let keypair = gen_keypair("testkey", false).unwrap();
        let shard = Shard::new(vec![1, 2, 3, 4], 1).unwrap();
        shard.encrypt(keypair.1).unwrap();
    }

    #[test]
    fn test_decrypt_shard() {
        let shard = Shard::new(vec![1, 2, 3, 4, 5], 1).unwrap();
        let keypair = gen_keypair("testkey", false).unwrap();
        let encrypted = shard.encrypt(keypair.1).unwrap();
        let decrypted = Shard::decrypt(encrypted, keypair.0).unwrap();
        assert_eq!(shard, decrypted);
    }

    #[test]
    fn test_gen_keypair() {
        gen_keypair("testkey", true).unwrap();
    }

    #[test]
    fn test_load_keypair() {}

    #[test]
    fn test_encrypt_decrypt_bytes() {
        let b = vec![1u8, 2, 3, 4, 5, 6, 5, 6, 7, 8, 7, 8, 9];
        let (pr, pu) = gen_keypair("key", false).unwrap();
        let encrypted = encrypt_bytes(&pu, &b).unwrap();
        let decrypted = decrypt_bytes(&pr, &encrypted).unwrap();
        assert_eq!(b, decrypted);
    }
}
