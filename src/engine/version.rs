#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

#[derive(Debug, Clone)]
pub struct LsmVersion {
    pub l0_sstables: Vec<usize>,
    pub levels: Vec<(usize, Vec<usize>)>,
}

impl LsmVersion {
    pub fn new() -> Self {
        Self {
            l0_sstables: Vec::new(),
            levels: Vec::new(),
        }
    }
}
