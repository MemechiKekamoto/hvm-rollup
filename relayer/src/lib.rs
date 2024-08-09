pub mod calldata;
pub mod commands;
pub mod connect;
pub mod relay;
pub mod runtime;
pub mod util;
pub mod error;
pub mod offchain_lab;
pub mod bend_program;

pub type Result<T> = std::result::Result<T, RelayerError>;

use serde::{Deserialize, Serialize};
use sp_core::{U256, H256};
pub use error::RelayerError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calldata {
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
    pub nonce: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BendProgram {
    pub id: H256,
    pub code: Vec<u8>,
    pub metadata: BendProgramMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BendProgramMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub total: usize,
    pub success: usize,
    pub error: usize,
}