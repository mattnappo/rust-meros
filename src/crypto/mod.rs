pub mod encryption;
pub mod hash;

use libp2p::identity;
use std::error::Error;
use std::fmt;

/// All of the errors that can be thrown by the Crypto module.
#[derive(Debug)]
pub enum CryptoError {
    SerializationError(bincode::Error),
    EncryptionError(ecies_ed25519::Error),
    IOError(std::io::Error),
    InvalidKey(crate::GeneralError),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CryptoError {}

/// Convert an ecies keypair into a libp2p keypair. Annoying.
pub fn ecies_to_libp2p(
    sk: &ecies_ed25519::SecretKey,
    pk: &ecies_ed25519::PublicKey,
) -> identity::Keypair {
    // Convert to a dalek key (the internal of a libp2p key)
    let dalek_pair = ed25519_dalek::Keypair {
        secret: ed25519_dalek::SecretKey::from_bytes(&sk.to_bytes()).unwrap(),
        public: ed25519_dalek::PublicKey::from_bytes(&pk.to_bytes()).unwrap(),
    };

    let libp2p_sk =
        identity::ed25519::SecretKey::from_bytes(dalek_pair.secret.to_bytes())
            .unwrap();

    identity::Keypair::Ed25519(identity::ed25519::Keypair::from(libp2p_sk)) // libp2p pair
}

/// Convert a ecies public key into a libp2p public key.
pub fn ecies_pub_to_libp2p(pk: &ecies_ed25519::PublicKey) -> identity::PublicKey {
    let dalek = ed25519_dalek::PublicKey::from_bytes(&pk.to_bytes()).unwrap();
    identity::PublicKey::Ed25519(
        identity::ed25519::PublicKey::decode(&dalek.to_bytes()).unwrap(),
    )
}
