pub mod database;

pub trait IsKey {}
pub trait IsValue {}

/// All of the errors that can be thrown in the database module.
#[derive(Debug)]
pub enum DatabaseError {
    Internal(sled::Error),
}

const ROOT: &str = "./data/db";
