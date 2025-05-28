#![allow(unused_variables)]
#![allow(dead_code)]

use crossbeam_skiplist::SkipMap;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::atomic::{ AtomicUsize, Ordering };

#[derive(Debug)]
pub struct MemTable {
    map: Arc<SkipMap<Bytes, Bytes>>,
    id: usize,
    size: AtomicUsize,
}

impl MemTable {
    pub fn new(id: usize) -> Self {
        Self {
            map: Arc::new(SkipMap::new()),
            id,
            size: AtomicUsize::new(0),
        }
    }

    pub fn put(&self, key: Bytes, value: Bytes) -> Result<(), String> {
        let entry_size = key.len() + value.len();
        self.map.insert(key, value);
        self.size.fetch_add(entry_size, Ordering::Relaxed);
        Ok(())
    }

    pub fn get(&self, key: &Bytes) -> Option<Bytes> {
        self.map.get(key).map(|entry| entry.value().clone())
    }

    pub fn approximate_size(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }
}
