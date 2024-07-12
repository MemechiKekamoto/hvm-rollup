mod batch;
mod transaction;

use crate::error::HVMError;
use crate::zk_rollup::{Proof, State};
use batch::Batch;
use transaction::Transaction;

pub struct Sequencer {
    state: State,
    pending_transactions: Vec<Transaction>,
}

impl Sequencer {
    pub fn new(initial_state: State) -> Self {
        Self {
            state: initial_state,
            pending_transactions: Vec::new(),
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<(), HVMError> {
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn create_batch(&mut self) -> Result<Batch, HVMError> {
        let batch = Batch::new(self.pending_transactions.clone());
        self.pending_transactions.clear();
        Ok(batch)
    }

    pub fn apply_proof(&mut self, proof: Proof) -> Result<(), HVMError> {
        self.state.apply_proof(&proof)
    }

    pub fn get_current_state(&self) -> State {
        self.state.clone()
    }
}