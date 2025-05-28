#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::{ Arc, RwLock, Mutex };
use bytes::Bytes;

use crate::engine::LsmEngine;

pub struct LsmStorage {
    engine: Arc<LsmEngine>,
}

impl LsmStorage {
    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        // TODO: delegate to engine.get
        todo!()
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        // TODO: delegate to engine.put
        todo!()
    }

    pub fn delete(&self, key: &[u8]) {
        // TODO: treat as tombstone via engine.put
        todo!()
    }

    pub fn flush(&self) {
        // TODO: trigger a flush (e.g., if memtable is full)
        todo!()
    }

    pub fn force_flush(&self) {
        // TODO: explicitly freeze + flush current memtable
        todo!()
    }
}
