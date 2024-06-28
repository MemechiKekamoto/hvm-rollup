use thiserror::Error;
use subxt::Error as SubxtError;

#[derive(Error, Debug)]
pub enum RelayerError {
    #[error("Subxt error: {0}")]
    SubxtError(#[from] SubxtError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Substrate error: {0}")]
    Substrate(String),

    #[error("Calldata fetch error: {0}")]
    CalldataFetchError(String),

    #[error("Extrinsic submission error: {0}")]
    ExtrinsicSubmissionError(String),

    #[error("Calldata verification failed")]
    CalldataVerificationFailed,

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Sequencer error: {0}")]
    SequencerError(String),
}