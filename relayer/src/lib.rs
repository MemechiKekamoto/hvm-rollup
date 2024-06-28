pub mod calldata;
pub mod connect;
pub mod relay;
pub mod runtime;
pub mod util;
pub mod error;

pub type Result<T> = std::result::Result<T, RelayerError>;

use serde::{Deserialize, Serialize};
use sp_core::U256;
pub use error::RelayerError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calldata {
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
    pub nonce: U256,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub total: usize,
    pub success: usize,
    pub error: usize,
}