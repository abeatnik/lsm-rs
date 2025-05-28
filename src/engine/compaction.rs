use anyhow::Result;

use super::engine::LsmEngineState;

#[derive(Debug, Clone)]
pub enum CompactionStyle {
    Leveled,
    Tiered,
    Simple,
    None,
}

#[derive(Debug)]
pub enum CompactionTask {
    Leveled(LevelCompactionTask),
    Tiered(TieredCompactionTask),
    Simple(SimpleCompactionTask),
    FullFlush,
}

impl CompactionTask {
    pub fn is_full_compaction(&self) -> bool {
        matches!(self, CompactionTask::FullFlush)
    }
}

pub enum CompactionController {
    Leveled(LeveledCompaction),
    Tiered(TieredCompaction),
    Simple(SimpleCompaction),
    None,
}

impl CompactionController {
    pub fn pick_compaction(&self, state: &LsmEngineState) -> Option<CompactionTask> {
        match self {
            CompactionController::Leveled(ctrl) => ctrl.pick(state),
            CompactionController::Tiered(ctrl) => ctrl.pick(state),
            CompactionController::Simple(ctrl) => ctrl.pick(state),
            CompactionController::None => None,
        }
    }

    pub fn apply_compaction(
        &self,
        state: &LsmEngineState,
        task: &CompactionTask
    ) -> Result<LsmEngineState> {
        match (self, task) {
            (CompactionController::Leveled(ctrl), CompactionTask::Leveled(t)) => {
                ctrl.apply(state, t)
            }
            (CompactionController::Tiered(ctrl), CompactionTask::Tiered(t)) => {
                ctrl.apply(state, t)
            }
            (CompactionController::Simple(ctrl), CompactionTask::Simple(t)) => {
                ctrl.apply(state, t)
            }
            _ => Err(anyhow::anyhow!("invalid compaction-task combination")),
        }
    }
}

#[derive(Debug)]
pub struct LeveledCompaction {
    pub max_levels: usize,
    pub target_file_size: usize,
}

impl LeveledCompaction {
    pub fn pick(&self, _state: &LsmEngineState) -> Option<CompactionTask> {
        // TODO: Analyze LSM version and choose SSTs to compact
        None
    }

    pub fn apply(
        &self,
        _state: &LsmEngineState,
        _task: &LevelCompactionTask
    ) -> Result<LsmEngineState> {
        // TODO: Merge SSTs, write new file, update version
        Ok(_state.clone())
    }
}

#[derive(Debug)]
pub struct TieredCompaction {
    pub max_tier: usize,
}

impl TieredCompaction {
    pub fn pick(&self, _state: &LsmEngineState) -> Option<CompactionTask> {
        None
    }

    pub fn apply(
        &self,
        _state: &LsmEngineState,
        _task: &TieredCompactionTask
    ) -> Result<LsmEngineState> {
        Ok(_state.clone())
    }
}

#[derive(Debug)]
pub struct SimpleCompaction;

impl SimpleCompaction {
    pub fn pick(&self, _state: &LsmEngineState) -> Option<CompactionTask> {
        None
    }

    pub fn apply(
        &self,
        _state: &LsmEngineState,
        _task: &SimpleCompactionTask
    ) -> Result<LsmEngineState> {
        Ok(_state.clone())
    }
}

// ------------------------ Task Structures --------------------------

#[derive(Debug)]
pub struct LevelCompactionTask {
    pub input_l0: Vec<usize>,
    pub input_l1: Vec<usize>,
    pub target_level: usize,
}

#[derive(Debug)]
pub struct TieredCompactionTask {
    pub input_sstables: Vec<usize>,
    pub output_tier: usize,
}

#[derive(Debug)]
pub struct SimpleCompactionTask {
    pub input_files: Vec<usize>,
}
