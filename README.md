# lsm-rs

A small LSM-tree-based NoSQL engine written in Rust.
LSM trees are the storage engine design behind systems like RocksDB, LevelDB, and Apache Cassandra.

This is a toy project, aiming to implement the core ideas behind log-structured merge-trees (LSM trees), including memtables, compaction, and eventually SSTables and WALs.
It is loosely modeled after [mini-lsm](https://github.com/skyzh/mini-lsm/blob/main/README.md).

Right now, it supports in-memory data structures and the skeleton for compaction logic.

## Features (work in progress)

- [x] In-memory memtables
- [x] Freezing active memtables to immutable ones
- [x] Basic compaction controller scaffolding
- [ ] Write-ahead logging
- [ ] SSTable writing + loading
- [ ] Range scans
- [ ] Compaction scheduling
- [ ] Concurrency and MVCC

## Getting started

```bash
git clone https://github.com/yourname/lsm-rs.git
cd lsm-rs
cargo build
cargo test
```
