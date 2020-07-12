use ecies_ed25519::{decrypt, encrypt, PublicKey, SecretKey};

use crate::CanSerialize;

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

pub trait CanEncrypt<T>
where
    T: CanSerialize,
{
    fn encrypt(
        &self,
        options: EncryptionOptions,
    ) -> Result<Vec<u8>, std::error::Error> {
        let mut csprng = rand::thread_rng();
        let mut bytes = <T as CanSerialize>::to_bytes()?;
        let encrypted = Vec::new();

        // Encrypt with a private key if there is as private key and a public key
        if let Some(key) = options.priv_key && let Some(_) = options.pub_key {
            let bytes = encrypt(&options.priv_key, bytes, &mut csprng)?;
        }

        // Encrypt with a public key if there is a public key
        if let Some(key) = options.pub_key {
            bytes = encrypt(&options.pub_key, bytes, &mut csprng)?;
        }
        encrypt();
    }
    fn decrypt(bytes: Vec<u8>) -> Self;
}
