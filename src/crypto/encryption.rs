use ecies_ed25519::{decrypt, encrypt, PublicKey, SecretKey};
use std::fs::File;

use crate::CanSerialize;

trait IsKey {}
impl IsKey for PublicKey {}
impl IsKey for SecretKey {}

/// All of the errors that can be thrown by the Crypto module.
pub enum CryptoError {
    SerializationError(bincode::Error),
    EncryptionError(ecies_ed25519::Error),
    IOError(std::io::Error),
}

/// Differnet ways to encrypt and decrypt data. This struct enables quick keypair
/// generation, and encryption and with a public key, private key, or both.
pub struct EncryptionOptions {
    pub_key: Option<PublicKey>, // Encrypt bytes with a public key
    priv_key: Option<SecretKey>, // Encrypt bytes with priv_key, then pub_key

    // Generate a new keypair, then encrypt/decrypt with that pub/priv key.
    // Export that key at the path stored in the string.
    gen_keypair: (bool, String),
}

impl EncryptionOptions {
    fn default_encrypt(key: &PublicKey) {
        EncryptionOptions {
            pub_key: Some(key),
            priv_key: None,
            gen_keypair: false,
        }
    }

    fn default_decrypt(key: &SecretKey) {
        EncryptionOptions {
            pub_key: None,
            Priv_key: Some(key),
            gen_keypair: false,
        }
    }
}

/// Write a key to the disk.
fn write_key<K>(keys: [K; 2], name: &str) -> Option<CryptoError>
where
    K: IsKey,
{
    for i in 0..2 {
        let key = bincode::serialize(&key[i])
            .map_err(|e| CryptoError::SerializationError(e))?;

        let mut file = File::create(
            format!(
                "./data/keys/{}.{}",
                name,
                match i {
                    0 => "priv",
                    1 => "pub",
                }
            )
            .as_str(),
        );
    }
}

/// Generate a public-private keypair and write to disk with the given name.
fn gen_keypair(name: &str) -> Result<(PublicKey, SecretKey), CryptoError> {
    let (priv_key, pub_key) = generate_keypair(&mut rand::thread_rng());
    write_keypair(priv_key)
}

pub trait CanEncrypt<T>
where
    T: CanSerialize,
{
    fn encrypt(
        &self,
        options: EncryptionOptions,
    ) -> Result<Vec<u8>, EncryptionError> {
        let mut csprng = rand::thread_rng();
        let mut bytes = <T as CanSerialize>::to_bytes()
            .map_err(|e| CryptoError::SerializationError(e))?;

        // Encrypt with a private key if there is as private key and a public key
        if let Some(key) = options.priv_key && let Some(_) = options.pub_key {
            let bytes: Vec<u8> = encrypt(&options.priv_key, bytes, &mut csprng).map_err(|e| CryptoError::EncryptionError(e))?;
        }

        if options.gen_keypair {
            gen_keypair()
        }

        // Encrypt with a public key if there is a public key
        if let Some(key) = options.pub_key {
            let bytes: Vec<u8> =
                encrypt(&options.pub_key, bytes, &mut csprng)?;
        }
    }

    fn decrypt(bytes: Vec<u8>) -> Self;
}
