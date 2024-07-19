use super::Proof;
use crate::error::HVMError;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct State {
    pub balance: u64,
    pub nonce: u64,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_proof(&mut self, proof: &Proof) -> Result<(), HVMError> {
        self.balance += proof.data.len() as u64;
        self.nonce += 1;
        println!("State updated: balance = {}, nonce = {}", self.balance, self.nonce);
        Ok(())
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}