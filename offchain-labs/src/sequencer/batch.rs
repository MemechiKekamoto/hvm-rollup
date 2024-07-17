use super::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    transactions: Vec<Transaction>,
    timestamp: u64,
}

impl Batch {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self {
            transactions,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}