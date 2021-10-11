use crate::*;

impl CommandService for Hget {
    fn execute(self, storage: &impl Storage) -> CommandResp {
        match storage.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}
