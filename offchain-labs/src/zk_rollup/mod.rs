mod proof;
mod state;

pub use proof::Proof;
pub use state::State;

use crate::error::HVMError;

pub trait ZKRollup {
    fn generate_proof(&self, state: &State, transaction: &[u8]) -> Result<Proof, HVMError>;
    fn verify_proof(&self, proof: &Proof) -> Result<bool, HVMError>;
    fn apply_proof(&self, state: &mut State, proof: &Proof) -> Result<(), HVMError>;
}