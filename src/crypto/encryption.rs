use ecies_ed25519::{
    decrypt, encrypt, generate_keypair, PublicKey, SecretKey,
};
use rand;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use crate::CanSerialize;

trait IsKey {}
impl IsKey for PublicKey {}
impl IsKey for SecretKey {}

const KEY_LOCATION: &str = "./data/keys/";

/// All of the errors that can be thrown by the Crypto module.
pub enum CryptoError {
    SerializationError(bincode::Error),
    EncryptionError(ecies_ed25519::Error),
    IOError(std::io::Error),
    NullKey(crate::GeneralError),
}

/// Specify the type of key and the name of the key.
enum KeyType {
    Public(String),
    Private(String),
}

/// A handy shorthand type representing a keypair.
type Keypair = (SecretKey, PublicKey);

/// Differnet ways to encrypt and decrypt data. This struct enables quick keypair
/// generation, and encryption and with a public key, private key, or both.
pub struct EncryptionOptions {
    pub_key: Option<PublicKey>, // Encrypt bytes with a public key
    priv_key: Option<SecretKey>, // Encrypt bytes with priv_key, then pub_key
}

impl EncryptionOptions {
    fn default_encrypt(key: PublicKey) -> EncryptionOptions {
        EncryptionOptions {
            pub_key: Some(key),
            priv_key: None,
        }
    }

    fn default_decrypt(key: SecretKey) -> EncryptionOptions {
        EncryptionOptions {
            pub_key: None,
            priv_key: Some(key),
        }
    }
}

/// Write a single key to the disk.
fn write_key<K>(key: &K, key_type: KeyType) -> Result<(), CryptoError>
where
    K: IsKey + serde::Serialize,
{
    let key = bincode::serialize(key)
        .map_err(|e| CryptoError::SerializationError(e))?;

    let (name, extension) = match key_type {
        KeyType::Private(name) => (name, "priv"),
        KeyType::Public(name) => (name, "pub"),
    };

    let mut file = File::create(
        format!("{}{}.{}", KEY_LOCATION, name, extension,).as_str(),
    )
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
    create_dir_all(Path::new(KEY_LOCATION))
        .map_err(|e| CryptoError::IOError(e))?;

    write_key(pair.0, KeyType::Private(name.to_string()))?;
    write_key(pair.1, KeyType::Public(name.to_string()))?;
    Ok(())
}

/// Generate a public-private keypair and write to disk with the given name.
fn gen_keypair(name: &str) -> Result<Keypair, CryptoError> {
    let mut csprng = rand::thread_rng();
    let (priv_key, pub_key) = generate_keypair(&mut csprng);
    write_keypair((&priv_key, &pub_key), name)?;
    Ok((priv_key, pub_key))
}

/*
fn load_key<K>(name: &str) -> Result<K, CryptoError>
where
    K: IsKey + serde::Serialize,
{

}

fn load_keypair(name: &str) -> Result<Keypair, CryptoError> {

}
*/

pub trait CanEncrypt: CanSerialize {
    fn encrypt(
        &self,
        options: EncryptionOptions,
    ) -> Result<Vec<u8>, CryptoError> {
        let mut csprng = rand::thread_rng();
        let bytes = self
            .to_bytes()
            .map_err(|e| CryptoError::SerializationError(e))?;
        let bytes = &bytes[..];

        if let Some(key) = options.pub_key {
            let bytes = &(encrypt(&key, bytes, &mut csprng)
                .map_err(|e| CryptoError::EncryptionError(e))?)[..];
        }

        Ok(bytes.to_vec())
    }

    fn decrypt<T>(
        bytes: Vec<u8>,
        options: EncryptionOptions,
    ) -> Result<T, CryptoError>
    where
        T: CanSerialize,
    {
        if let Some(key) = options.priv_key {
            let decrypted = decrypt(&key, &bytes[..])
                .map_err(|e| CryptoError::EncryptionError(e))?;

            return match T::from_bytes(bytes) {
                Ok(reconstructed) => Ok(reconstructed),
                Err(e) => Err(CryptoError::SerializationError(e)),
            };
            //.map_err(|e| CryptoError::SerializationError(e))?;
        }

        Err(CryptoError::NullKey(crate::GeneralError::new(
            "cannot decrypt with a null private key",
        )))
    }
}
