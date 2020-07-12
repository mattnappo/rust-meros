pub trait IsKey {}
pub trait IsValue {}

pub struct Database<K: IsKey, V: IsValue> {
    database: Vec<u8>, // temp
    lock: std::sync::Mutex,
}
