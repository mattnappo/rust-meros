use crate::crypto;
use std::error::Error;
use std::fmt;

pub mod file;

/// A trait given to types that are able to be hashed.
trait Hashable {
    fn hash(&self) -> crypto::Hash;
}

#[derive(Debug)]
struct GenericError {
    details: String,
}

impl GenericError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}
