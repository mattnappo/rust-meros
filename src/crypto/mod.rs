pub mod encryption;
pub mod hash;

use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

/// All of the errors that can be thrown by the Crypto module.
#[derive(Debug)]
pub enum CryptoError {
    SerializationError(bincode::Error),
    EncryptionError(ecies_ed25519::Error),
    IOError(std::io::Error),
    InvalidKey(crate::GeneralError),
}
