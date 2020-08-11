pub mod database;

const ROOT: &str = "./data/db";

/// All of the errors that can be thrown in the database module.
#[derive(Debug)]
pub enum DatabaseError {
    Internal(sled::Error),
}

pub trait IsKey {} // Get rid of these
pub trait IsValue {}

// TODO: Add lifetimes for this
/// A key-value pair for the database.
pub struct Pair<K: IsKey, V: IsValue> {
    k: &K,
    v: &V,
}
