use crate::error::HVMError;
use crate::bend::BendProgram;
use crate::sequencer::Batch;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Storage {
    programs: Arc<RwLock<HashMap<String, BendProgram>>>,
    batches: Arc<RwLock<HashMap<u64, Batch>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            programs: Arc::new(RwLock::new(HashMap::new())),
            batches: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn store_program(&self, program: BendProgram) -> Result<(), HVMError> {
        let program_id = program.id().to_string();
        self.programs.write().map_err(|_| HVMError::StorageLock("Failed to acquire write lock".to_string()))?.insert(program_id, program);
        Ok(())
    }

    pub fn load_program(&self, program_id: &str) -> Result<BendProgram, HVMError> {
        self.programs.read().map_err(|_| HVMError::StorageLock("Failed to acquire read lock".to_string()))?
            .get(program_id)
            .cloned()
            .ok_or_else(|| HVMError::ProgramNotFound(format!("Program not found: {}", program_id)))
    }

    pub fn store_batch(&self, batch: Batch) -> Result<(), HVMError> {
        let batch_id = batch.batch_id();
        self.batches.write().map_err(|_| HVMError::StorageLock("Failed to acquire write lock".to_string()))?.insert(batch_id, batch);
        Ok(())
    }
}