use offchain_labs::{Config, OffchainLabs};
use offchain_labs::sequencer::Transaction;
use offchain_labs::config::{ProverConfig, VerifierConfig, SequencerConfig};
use offchain_labs::bend::BendProgram;
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

#[tokio::test]
async fn test_prover_generate_proof() {
    let config = create_test_config();
    let mut hvm = OffchainLabs::new(config).unwrap();

    let transactions = vec![
        Transaction::new("Alice".to_string(), "Bob".to_string(), vec![100], 1, "test_program".to_string()),
        Transaction::new("Bob".to_string(), "Charlie".to_string(), vec![50], 2, "test_program".to_string()),
        Transaction::new("Charlie".to_string(), "Alice".to_string(), vec![25], 3, "test_program".to_string()),
    ];

    for (i, tx) in transactions.into_iter().enumerate() {
        let result = hvm.process_transaction(tx);
        assert!(result.is_ok(), "Failed to process transaction {}: {:?}", i, result.err());
        let is_valid = result.unwrap();
        assert!(is_valid, "Transaction {} was invalid", i);
    }

    let final_state = hvm.get_current_state().unwrap();
    assert_eq!(final_state.balance(), 768, "Unexpected final balance");
    assert_eq!(final_state.nonce(), 3, "Unexpected final nonce");
}

#[test]
fn test_prover_resource_estimation() {
    let config = create_test_config();
    let hvm = OffchainLabs::new(config).unwrap();

    let program = BendProgram::new(
        vec![1, 2, 3, 4],
        offchain_labs::bend::ProgramMetadata {
            name: "Test Program".to_string(),
            version: "1.0.0".to_string(),
            description: "Bend program for test".to_string(),
        },
        "Test Author".to_string(),
    );

    let usage = hvm.estimate_program_resources(&program);
    assert!(usage.is_ok(), "Failed to estimate resource usage");
    let usage = usage.unwrap();
    assert!(usage.cpu_cycles > 0, "CPU cycles should be positive");
    assert!(usage.memory_usage > 0, "Memory usage should be positive");
}

#[test]
fn test_prover_program_optimization() {
    let config = create_test_config();
    let hvm = OffchainLabs::new(config).unwrap();

    let program = BendProgram::new(
        vec![1, 2, 3, 4],
        offchain_labs::bend::ProgramMetadata {
            name: "Test Program".to_string(),
            version: "1.0.0".to_string(),
            description: "Bend program for test".to_string(),
        },
        "Test Author".to_string(),
    );

    let optimized = hvm.optimize_program(&program);
    assert!(optimized.is_ok(), "Failed to optimize program");
    let optimized = optimized.unwrap();
    assert_ne!(program.id(), optimized.id(), "Optimized program should have a different ID");
}