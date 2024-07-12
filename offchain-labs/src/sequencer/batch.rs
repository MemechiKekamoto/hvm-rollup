use super::transaction::Transaction;

pub struct Batch {
    transactions: Vec<Transaction>,
}

impl Batch {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self { transactions }
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}