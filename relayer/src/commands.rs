use offchain_labs::prover::ZKProver;
use offchain_labs::sequencer::Sequencer;
use offchain_labs::verifier::ZKVerifier;
use offchain_labs::bend::{BendProgram, ProgramMetadata, storage::Storage};

use offchain_labs::{
    Config, OffchainLabs,
    config::{self, ProverConfig, VerifierConfig, SequencerConfig},
    zk_rollup::{State, Proof},
};

use std::path::{Path, PathBuf};
use std::fs;

pub fn upload_bend_program() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Storage::new();
    let program = BendProgram::new(
        vec![1, 2, 3, 4],
        ProgramMetadata {
            name: "Test Program".to_string(),
            version: "1.0.0".to_string(),
            description: "Bend program for test".to_string(),
        },
        "Test Author".to_string(),
    );
    storage.store_program(program)?;

    Ok(())
}

pub fn process_batch() -> Result<(), Box<dyn std::error::Error>> {
    let config = SequencerConfig {
        max_pending_transactions: 5,
        max_pending_programs: 3,
        batch_interval_seconds: 1,
        max_batch_size: 3,
        max_programs_per_batch: 2,
    };

    let mut sequencer = Sequencer::new(State::default(), config);

    let batch = sequencer.create_batch(true)?;
    println!("Batch processed: {:?}", batch);
    Ok(())
}

pub fn verify_and_submit() -> Result<(), Box<dyn std::error::Error>> {
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
            max_pending_programs: 50,
            batch_interval_seconds: 10,
            max_batch_size: 50,
            max_programs_per_batch: 25,
        },
    };

    let hvm = OffchainLabs::new(config);
    assert!(hvm.is_ok());

    Ok(())
}