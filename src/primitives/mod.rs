use std::fmt;
use std::error::Error;

pub mod file;

/// The default size of hashes
const HASH_SIZE: u8; 32;

/// A type alias for Hashes to be used in the `primitives` module.
type Hash = [u8; HASH_SIZE];

/// A trait given to types that are able to be hashed.
trait Hashable {
    fn hash(&self) -> Vec<u8>;
}

#[derive(Debug)]
struct Error {
    details: String
}

impl Error {
    fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
