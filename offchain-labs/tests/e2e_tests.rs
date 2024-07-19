use offchain_labs::{Config, OffchainLabs};
use offchain_labs::sequencer::Transaction;
use offchain_labs::config::{ProverConfig, VerifierConfig, SequencerConfig};
use std::path::PathBuf;

#[tokio::test]
async fn test_end_to_end_workflow() {
    let config = Config {
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
            batch_interval_seconds: 1,
            max_batch_size: 50,
        },
    };

    let mut hvm = OffchainLabs::new(config).unwrap();
    let transactions = vec![
        Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1),
        Transaction::new("Bob".to_string(), "Charlie".to_string(), 50, 2),
        Transaction::new("Charlie".to_string(), "Alice".to_string(), 25, 3),
    ];

    for (i, tx) in transactions.into_iter().enumerate() {
        let result = hvm.process_transaction(tx);
        assert!(result.is_ok(), "Failed to process transaction {}: {:?}", i, result.err());
        let is_valid = result.unwrap();
        assert!(is_valid, "Transaction {} was invalid", i);
        
        let current_state = hvm.get_current_state().unwrap();
        println!("State after transaction {}: {:?}", i, current_state);
    }

    let final_state = hvm.get_current_state().unwrap();
    println!("Final state: {:?}", final_state);
    assert_eq!(final_state.balance(), 12, "Unexpected final balance");
    assert_eq!(final_state.nonce(), 3, "Unexpected final nonce");
}