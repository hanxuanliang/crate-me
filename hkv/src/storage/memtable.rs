use dashmap::{DashMap, mapref::one::Ref};

use crate::{Storage, Value};

#[derive(Debug, Default, Clone)]
pub struct Memtable {
    tables: DashMap<String, DashMap<String, Value>>
}

impl Memtable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for Memtable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, crate::KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }
}


#[cfg(test)]
mod tests {
    use crate::Storage;

    use super::Memtable;

    #[test]
    fn memtable_trait_iswork() {
        let store = Memtable::new();
        test_basic_traie(store);
    }

    fn test_basic_traie(store: impl Storage) {

    }
}
