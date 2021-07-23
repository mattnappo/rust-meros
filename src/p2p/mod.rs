pub mod client;
pub mod handler;
pub mod identity;
pub mod node;
pub mod store;

/// The maximum number of shards of a singular file.
pub const MAX_SHARDS: usize = 15;
