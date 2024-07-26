use offchain_labs::{Config, OffchainLabs};
use offchain_labs::config::{ProverConfig, VerifierConfig, SequencerConfig};
use offchain_labs::sequencer::Transaction;
use std::path::PathBuf;

fn create_test_config() -> Config {
    Config {
        zk_params_path: PathBuf::from("test_params.json"),
        state_db_path: PathBuf::from("test_state.db"),
        prover_config: ProverConfig {
            proving_key_path: PathBuf::from("test_proving_key.bin"),
            max_batch_size: 10,
        },
        verifier_config: VerifierConfig {
            verification_key_path: PathBuf::from("test_verification_key.bin"),
        },
        sequencer_config: SequencerConfig {
            max_pending_transactions: 100,
            max_pending_programs: 50,
            batch_interval_seconds: 10,
            max_batch_size: 50,
            max_programs_per_batch: 25,
        },
    }
}

#[test]
fn test_offchain_labs_initialization() {
    let config = create_test_config();
    let hvm = OffchainLabs::new(config);
    assert!(hvm.is_ok());
}

#[test]
fn test_transaction_processing() {
    let config = create_test_config();
    let mut hvm = OffchainLabs::new(config).unwrap();
    let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1);
    let result = hvm.process_transaction(transaction);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_transactions() {
    let config = create_test_config();
    let mut hvm = OffchainLabs::new(config).unwrap();
    let transactions = vec![
        Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1),
        Transaction::new("Bob".to_string(), "Charlie".to_string(), 50, 1),
        Transaction::new("Charlie".to_string(), "Alice".to_string(), 25, 1),
    ];

    for (i, tx) in transactions.into_iter().enumerate() {
        let result = hvm.process_transaction(tx);
        assert!(result.is_ok(), "Failed to process transaction {}", i);
    }

    assert_eq!(hvm.processed_transactions_count(), 3, "Expected 3 processed transactions");
    assert_eq!(hvm.pending_transactions_count(), 0, "Expected 0 pending transactions");

    let final_state = hvm.get_current_state().unwrap();
    println!("Final state: {:?}", final_state);
    assert_eq!(final_state.balance(), 768, "Unexpected final balance");
    assert_eq!(final_state.nonce(), 3, "Unexpected final nonce");

    println!("Processed transactions: {:?}", hvm.get_processed_transactions());
    println!("Pending transactions: {:?}", hvm.get_pending_transactions());
}

#[test]
fn test_zk_snark_proof_generation_and_verification() {
    let config = create_test_config();
    let mut hvm = OffchainLabs::new(config).unwrap();
    
    let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1);
    let result = hvm.process_transaction(transaction);
    assert!(result.is_ok());
    
    let is_valid = result.unwrap();
    assert!(is_valid, "Proof verification failed");
    
    assert_eq!(hvm.processed_transactions_count(), 1, "Expected 1 processed transaction");
    assert_eq!(hvm.pending_transactions_count(), 0, "Expected 0 pending transactions");
}