mod libs;

use crate::error::HVMError;
use crate::zk_rollup::Proof;
use libs::ZKSnarkLibs;

pub struct Prover {
    strategy: Box<dyn ProverLibs>,
}

impl Prover {
    pub fn new(strategy: Box<dyn ProverLibs>) -> Self {
        Self { strategy }
    }

    pub fn generate_proof(&self, input: &[u8]) -> Result<Proof, HVMError> {
        self.strategy.generate_proof(input)
    }
}

pub trait ProverLibs {
    fn generate_proof(&self, input: &[u8]) -> Result<Proof, HVMError>;
}

pub fn create_zk_snark_prover() -> Prover {
    Prover::new(Box::new(ZKSnarkLibs::new()))
}