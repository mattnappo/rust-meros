use std::fmt;

pub mod crypto;
pub mod db;
pub mod primitives;

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

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}
