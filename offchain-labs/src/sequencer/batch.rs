use super::transaction::Transaction;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    transactions: Vec<Transaction>,
    timestamp: u64,
    batch_id: u64,
}

impl Batch {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        static BATCH_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let batch_id = BATCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self {
            transactions,
            timestamp,
            batch_id,
        }
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn batch_id(&self) -> u64 {
        self.batch_id
    }
}