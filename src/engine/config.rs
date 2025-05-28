use crate::engine::CompactionStyle;

#[derive(Debug, Clone)]
pub struct LsmEngineConfig {
    /// Block size in bytes (used for SSTable block layout)
    pub block_size: usize,

    /// Target size for flushing memtable to SST (in bytes)
    pub target_sst_size: usize,

    /// Maximum number of immutable memtables allowed before forced flush
    pub num_memtable_limit: usize,

    /// Compaction strategy and parameters
    pub compaction: CompactionStyle,

    /// Whether to write a WAL for durability
    pub enable_wal: bool,

    /// Whether reads/writes must follow serializability (MVCC-related)
    pub serializable: bool,
}

impl Default for LsmEngineConfig {
    fn default() -> Self {
        Self {
            block_size: 4096,
            target_sst_size: 2 << 20, // 2MB
            num_memtable_limit: 50,
            compaction: CompactionStyle::None,
            enable_wal: false,
            serializable: false,
        }
    }
}

impl LsmEngineConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
