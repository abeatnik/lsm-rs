#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod
use bytes::Bytes;
use std::sync::{ Arc, RwLock, Mutex };
use std::sync::atomic::{ AtomicUsize, Ordering };

pub use crate::engine::{ MemtableSet, LsmVersion, MemTable, CompactionStyle, LsmEngineConfig };

const MEMTABLE_FLUSH_THRESHOLD: usize = 4 * 1024 * 1024; //4MB, should be more later, maybe 256MB?

#[derive(Debug, Clone)]
pub struct LsmEngineState {
    pub memtables: Arc<MemtableSet>,
    pub version: Arc<LsmVersion>, // SSTables
}

impl LsmEngineState {
    pub fn new() -> Self {
        Self {
            memtables: Arc::new(MemtableSet::new()),
            version: Arc::new(LsmVersion::new()),
        }
    }
}

pub struct LsmEngine {
    state: Arc<RwLock<Arc<LsmEngineState>>>,
    state_lock: Mutex<()>,
    flush_lock: Mutex<()>, //freeze memtables safely
    next_memtable_id: AtomicUsize,
    pub config: Arc<LsmEngineConfig>,
}

impl LsmEngine {
    pub fn new(config: LsmEngineConfig) -> Self {
        let state = Arc::new(LsmEngineState::new());
        Self {
            state: Arc::new(RwLock::new(state)),
            state_lock: Mutex::new(()),
            flush_lock: Mutex::new(()),
            next_memtable_id: AtomicUsize::new(1), // memtable 0 is created in LsmEngineState::new
            config: Arc::new(config),
        }
    }

    pub(crate) fn next_memtable_id(&self) -> usize {
        self.next_memtable_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        let state = self.state.read().expect("state lock poisoned");
        state.memtables.get(key)
    }

    pub fn put(&self, key: Bytes, value: Bytes) {
        let state = self.state.read().expect("state lock poisoned");
        let memtable = &state.memtables.memtable;
        memtable.put(key, value).unwrap();

        let _guard = self.state_lock.lock();
        let state = self.state.read().expect("state lock poisoned");
        if state.memtables.memtable.approximate_size() >= MEMTABLE_FLUSH_THRESHOLD {
            self.force_freeze_memtable();
        }
    }

    pub fn force_freeze_memtable(&self) {
        let _flush_guard = self.flush_lock.lock();

        let state = self.state.read().expect("state lock poisoned");
        let memtable = &state.memtables.memtable;

        if memtable.approximate_size() < MEMTABLE_FLUSH_THRESHOLD {
            return;
        }

        let new_id = self.next_memtable_id.fetch_add(1, Ordering::SeqCst);
        let new_active = Arc::new(MemTable::new(new_id));

        let mut new_imm = vec![memtable.clone()];
        new_imm.extend(state.memtables.imm_memtables.iter().cloned());

        let new_memtables = Arc::new(MemtableSet {
            memtable: new_active,
            imm_memtables: new_imm,
        });

        let new_state = Arc::new(LsmEngineState {
            memtables: new_memtables,
            version: state.version.clone(),
        });

        drop(state);
        *self.state.write().expect("lock poisoned") = new_state;
    }

    pub fn flush_imm_memtables(&self) {
        // TODO: flush oldest immutable memtable to SST
        todo!()
    }

    pub fn force_compaction(&self) {
        // TODO: merge SSTables (L0 → L1, etc.)
        todo!()
    }
}
