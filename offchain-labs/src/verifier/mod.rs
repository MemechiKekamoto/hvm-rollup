mod libs;

use crate::error::HVMError;
use crate::zk_rollup::Proof;
use libs::ZKSnarkLibs;

pub struct Verifier {
    strategy: Box<dyn VerifierLibs>,
}

impl Verifier {
    pub fn new(strategy: Box<dyn VerifierLibs>) -> Self {
        Self { strategy }
    }

    pub fn verify_proof(&self, proof: &Proof) -> Result<bool, HVMError> {
        self.strategy.verify_proof(proof)
    }
}

pub trait VerifierLibs {
    fn verify_proof(&self, proof: &Proof) -> Result<bool, HVMError>;
}

pub fn create_zk_snark_verifier() -> Verifier {
    Verifier::new(Box::new(ZKSnarkLibs::new()))
}