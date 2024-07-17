mod batch;
mod transaction;

use crate::error::HVMError;
use crate::zk_rollup::{Proof, State};
use crate::config::SequencerConfig;
pub use batch::Batch;
pub use transaction::Transaction;
use std::time::{Duration, Instant};

pub struct Sequencer {
    state: State,
    pending_transactions: Vec<Transaction>,
    config: SequencerConfig,
    last_batch_time: Instant,
}

impl Sequencer {
    pub fn new(initial_state: State, config: SequencerConfig) -> Self {
        Self {
            state: initial_state,
            pending_transactions: Vec::new(),
            config,
            last_batch_time: Instant::now(),
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<(), HVMError> {
        if self.pending_transactions.len() >= self.config.max_pending_transactions {
            return Err(HVMError::Sequencer("Max pending transactions reached".to_string()));
        }
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn create_batch(&mut self) -> Result<Option<Batch>, HVMError> {
        if self.pending_transactions.is_empty() {
            return Ok(None);
        }

        let now = Instant::now();
        if now.duration_since(self.last_batch_time) < Duration::from_secs(self.config.batch_interval_seconds) {
            return Ok(None);
        }

        let batch = Batch::new(self.pending_transactions.clone());
        self.pending_transactions.clear();
        self.last_batch_time = now;
        Ok(Some(batch))
    }

    pub fn apply_proof(&mut self, proof: Proof) -> Result<(), HVMError> {
        self.state.apply_proof(&proof)
    }

    pub fn get_current_state(&self) -> State {
        self.state.clone()
    }

    pub fn pending_transactions_count(&self) -> usize {
        self.pending_transactions.len()
    }
}