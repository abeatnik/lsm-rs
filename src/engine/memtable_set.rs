#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::Arc;
use bytes::Bytes;

use super::MemTable;

#[derive(Debug, Clone)]
pub struct MemtableSet {
    //only one active memtable, will be frozen when size limit is reached, i.e. 256MB
    pub memtable: Arc<MemTable>,
    pub imm_memtables: Vec<Arc<MemTable>>,
}

impl MemtableSet {
    pub fn new() -> Self {
        Self {
            memtable: Arc::new(MemTable::new(0)),
            imm_memtables: Vec::new(), //[ newest, ..., oldest ]
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        let key_bytes = Bytes::copy_from_slice(key);

        if let Some(val) = self.memtable.get(&key_bytes) {
            return Some(val);
        }

        for imm in &self.imm_memtables {
            if let Some(val) = imm.get(&key_bytes) {
                return Some(val);
            }
        }

        None
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), String> {
        let key = Bytes::copy_from_slice(key);
        let value = Bytes::copy_from_slice(value);
        self.memtable.put(key, value)
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), String> {
        let key = Bytes::copy_from_slice(key);
        self.memtable.put(key, Bytes::new())
    }
}
