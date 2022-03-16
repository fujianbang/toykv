use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

#[derive(Clone, Debug, Default)]
pub struct MemTable {
    table: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.table.get(name) {
            Some(table) => table,
            None => {
                let entry = self.table.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {}