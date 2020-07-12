/// The different types of data that a `Database` can hold.
pub enum DBType {
    File,
    Shard,
}

/// The main database structure used to store the metadata and node
/// information for all of the files on the network, and the shards
/// on each local node.
pub struct Database<K: super::IsKey, V: super::IsValue> {
    database: Vec<u8>, // temp
    db_type: DBType,
    default_key: K,
    default_value: V,
}

impl<K: super::IsKey, V: super::IsValue> Database<K, V> {
    fn new(db_type: DBType, dk: K, dv: V) -> Self {
        Self {
            database: Vec::new(),
            db_type,
            default_key: dk,
            default_value: dv,
        }
    }
}
