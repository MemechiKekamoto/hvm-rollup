use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;

pub struct Prover;

impl Prover {
    pub fn generate_proof(&self, _batch: &Batch) -> Result<Proof, HVMError> {
        Ok(Proof::new(vec![1, 2, 3, 4]))
    }
}

pub fn create_zk_snark_prover() -> Prover {
    Prover
}