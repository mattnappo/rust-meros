pub mod database;
use crate::CanSerialize;

const ROOT: &str = "./data/db";

/// All of the errors that can be thrown in the database module.
#[derive(Debug)]
pub enum DatabaseError {
    Internal(sled::Error),
    Serialize(bincode::Error),
}

/// The bounding trait representing any type that can act as a
/// key in the database.
pub trait IsKey: CanSerialize {}

/// The bounding trait representing any type that can act as a
/// value in the database.
pub trait IsValue: CanSerialize {}
