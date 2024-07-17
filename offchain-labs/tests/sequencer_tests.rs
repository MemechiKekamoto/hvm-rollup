use offchain_labs::{
    config::{Config, SequencerConfig},
    sequencer::{Sequencer, Transaction},
    zk_rollup::State,
};
use std::time::Duration;

fn create_test_sequencer() -> Sequencer {
    let config = SequencerConfig {
        max_pending_transactions: 5,
        batch_interval_seconds: 1,
    };
    Sequencer::new(State::default(), config)
}

#[test]
fn test_process_transaction() {
    let mut sequencer = create_test_sequencer();
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1);
    assert!(sequencer.process_transaction(tx).is_ok());
    assert_eq!(sequencer.pending_transactions_count(), 1);
}

#[test]
fn test_max_pending_transactions() {
    let mut sequencer = create_test_sequencer();
    for i in 0..5 {
        let tx = Transaction::new(format!("Sender{}", i), format!("Recipient{}", i), 100, 1);
        assert!(sequencer.process_transaction(tx).is_ok());
    }
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1);
    assert!(sequencer.process_transaction(tx).is_err());
}

#[test]
fn test_create_batch() {
    let mut sequencer = create_test_sequencer();
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1);
    sequencer.process_transaction(tx).unwrap();

    assert!(sequencer.create_batch().unwrap().is_none());

    std::thread::sleep(Duration::from_secs(1));
    
    let batch = sequencer.create_batch().unwrap();
    assert!(batch.is_some());
    assert_eq!(batch.unwrap().transactions().len(), 1);
    assert_eq!(sequencer.pending_transactions_count(), 0);
}