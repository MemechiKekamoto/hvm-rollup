mod batch;
mod transaction;

use crate::error::HVMError;
use crate::zk_rollup::{Proof, State};
use crate::config::SequencerConfig;
pub use batch::Batch;
pub use transaction::Transaction;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct Sequencer {
    state: State,
    pending_transactions: VecDeque<Transaction>,
    processed_transactions: Vec<Transaction>,
    config: SequencerConfig,
    last_batch_time: Instant,
}

impl Sequencer {
    pub fn new(initial_state: State, config: SequencerConfig) -> Self {
        Self {
            state: initial_state,
            pending_transactions: VecDeque::new(),
            processed_transactions: Vec::new(),
            config,
            last_batch_time: Instant::now(),
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<(), HVMError> {
        if self.pending_transactions.len() >= self.config.max_pending_transactions {
            return Err(HVMError::Sequencer("Max pending transactions reached".to_string()));
        }
        self.pending_transactions.push_back(transaction);
        Ok(())
    }

    pub fn create_batch(&mut self, force: bool) -> Result<Option<Batch>, HVMError> {
        if self.pending_transactions.is_empty() {
            return Ok(None);
        }
    
        let now = Instant::now();
        if !force && now.duration_since(self.last_batch_time) < Duration::from_secs(self.config.batch_interval_seconds) {
            return Ok(None);
        }
    
        let mut batch_transactions = Vec::new();
        while let Some(tx) = self.pending_transactions.pop_front() {
            batch_transactions.push(tx);
            if batch_transactions.len() >= self.config.max_batch_size {
                break;
            }
        }
    
        let batch = Batch::new(batch_transactions);
        self.last_batch_time = now;
        Ok(Some(batch))
    }

    pub fn apply_proof(&mut self, proof: Proof, batch: &Batch) -> Result<(), HVMError> {
        println!("Applying proof in sequencer: {:?}", proof);
        let result = self.state.apply_proof(&proof);
        println!("State after applying proof: {:?}", self.state);
        
        for tx in batch.transactions() {
            self.processed_transactions.push(tx.clone());
        }
        
        result
    }

    pub fn get_current_state(&self) -> State {
        self.state.clone()
    }

    pub fn pending_transactions_count(&self) -> usize {
        self.pending_transactions.len()
    }

    pub fn processed_transactions_count(&self) -> usize {
        self.processed_transactions.len()
    }

    pub fn get_pending_transactions(&self) -> &VecDeque<Transaction> {
        &self.pending_transactions
    }

    pub fn get_processed_transactions(&self) -> &Vec<Transaction> {
        &self.processed_transactions
    }
}