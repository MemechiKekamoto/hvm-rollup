#[warn(unused_imports)]
use offchain_labs::{
    config::SequencerConfig,
    sequencer::{Sequencer, Transaction},
    zk_rollup::{State, Proof},
};

fn create_test_sequencer() -> Sequencer {
    let config = SequencerConfig {
        max_pending_transactions: 5,
        max_pending_programs: 3,
        batch_interval_seconds: 1,
        max_batch_size: 3,
        max_programs_per_batch: 2,
    };
    Sequencer::new(State::default(), config)
}

#[test]
fn test_process_transaction() {
    let mut sequencer = create_test_sequencer();
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), vec![100], 1, "test_program".to_string());
    assert!(sequencer.process_transaction(tx).is_ok());
    assert_eq!(sequencer.pending_transactions_count(), 1);
}

#[test]
fn test_max_pending_transactions() {
    let mut sequencer = create_test_sequencer();
    for i in 0..5 {
        let tx = Transaction::new(format!("Sender{}", i), format!("Recipient{}", i), vec![100], 1, "test_program".to_string());
        assert!(sequencer.process_transaction(tx).is_ok());
    }
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), vec![100], 1, "test_program".to_string());
    assert!(sequencer.process_transaction(tx).is_err());
}

#[test]
fn test_create_batch() {
    let mut sequencer = create_test_sequencer();

    for i in 0..4 {
        let tx = Transaction::new(format!("Sender{}", i), format!("Recipient{}", i), vec![100], 1, "test_program".to_string());
        sequencer.process_transaction(tx).unwrap();
    }

    let first_batch = sequencer.create_batch(true).unwrap();
    assert!(first_batch.is_some(), "Expected a batch to be created");
    let first_batch = first_batch.unwrap();
    assert_eq!(first_batch.transactions().len(), 3, "Expected 3 transactions in the first batch");
    assert_eq!(sequencer.pending_transactions_count(), 1, "Expected 1 pending transaction after first batch");
    
    let second_batch = sequencer.create_batch(true).unwrap();
    assert!(second_batch.is_some(), "Expected a second batch to be created");
    let second_batch = second_batch.unwrap();
    assert_eq!(second_batch.transactions().len(), 1, "Expected 1 transaction in the second batch");
    assert_eq!(sequencer.pending_transactions_count(), 0, "Expected 0 pending transactions after second batch");
    
    let third_batch = sequencer.create_batch(true).unwrap();
    assert!(third_batch.is_none(), "Expected no batch to be created when there are no pending transactions");

    assert!(second_batch.batch_id() > first_batch.batch_id(), "Expected second batch ID to be greater than first batch ID");
}

#[test]
fn test_apply_proof() {
    let mut sequencer = create_test_sequencer();
    let initial_state = sequencer.get_current_state();
    
    for i in 0..3 {
        let tx = Transaction::new(format!("Sender{}", i), format!("Recipient{}", i), vec![100], 1, "test_program".to_string());
        sequencer.process_transaction(tx).unwrap();
    }
    let batch = sequencer.create_batch(true).unwrap().unwrap();
    
    let proof = Proof::new(vec![1, 2, 3, 4]);
    
    assert!(sequencer.apply_proof(proof, &batch).is_ok());
    
    let new_state = sequencer.get_current_state();
    assert!(new_state.nonce() > initial_state.nonce());
    assert_eq!(sequencer.processed_transactions_count(), 3);
    assert_eq!(sequencer.pending_transactions_count(), 0);
}