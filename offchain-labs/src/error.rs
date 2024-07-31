use thiserror::Error;

#[derive(Error, Debug)]
pub enum HVMError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Prover error: {0}")]
    Prover(String),

    #[error("Verifier error: {0}")]
    Verifier(String),

    #[error("Sequencer error: {0}")]
    Sequencer(String),

    #[error("ZK Rollup error: {0}")]
    ZKRollup(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Setup error: {0}")]
    Setup(String),

    #[error("Storage error: {0}")]
    StorageLock(String),

    #[error("Program error: {0}")]
    ProgramNotFound(String),
}