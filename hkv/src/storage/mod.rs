mod memtable;

pub use memtable::Memtable;

use crate::{
    Value, KvError
};

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
}
