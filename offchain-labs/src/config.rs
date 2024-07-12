use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::error::HVMError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub zk_params_path: PathBuf,
    pub state_db_path: PathBuf,
    pub prover_config: ProverConfig,
    pub verifier_config: VerifierConfig,
    pub sequencer_config: SequencerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverConfig {
    pub proving_key_path: PathBuf,
    pub max_batch_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifierConfig {
    pub verification_key_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequencerConfig {
    pub max_pending_transactions: usize,
    pub batch_interval_seconds: u64,
}

impl Config {
    pub fn load() -> Result<Self, HVMError> {
        let mut file = File::open("config.json").map_err(|e| HVMError::Config(format!("Failed to open config file: {}", e)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| HVMError::Config(format!("Failed to read config file: {}", e)))?;
        serde_json::from_str(&contents).map_err(|e| HVMError::Config(format!("Failed to parse config file: {}", e)))
    }

    pub fn save(&self) -> Result<(), HVMError> {
        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| HVMError::Config(format!("Failed to serialize config: {}", e)))?;
        std::fs::write("config.json", contents)
            .map_err(|e| HVMError::Config(format!("Failed to write config file: {}", e)))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            zk_params_path: PathBuf::from("zk_params.json"),
            state_db_path: PathBuf::from("state.db"),
            prover_config: ProverConfig {
                proving_key_path: PathBuf::from("proving_key.bin"),
                max_batch_size: 100,
            },
            verifier_config: VerifierConfig {
                verification_key_path: PathBuf::from("verification_key.bin"),
            },
            sequencer_config: SequencerConfig {
                max_pending_transactions: 1000,
                batch_interval_seconds: 60,
            },
        }
    }
}