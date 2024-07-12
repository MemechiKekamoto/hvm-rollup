use super::Proof;
use crate::error::HVMError;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct State {
    pub balance: u64,
    pub nonce: u64,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_proof(&mut self, proof: &Proof) -> Result<(), HVMError> {
        self.balance += 1;
        self.nonce += 1;
        Ok(())
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}