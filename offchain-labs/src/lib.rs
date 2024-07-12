pub mod config;
pub mod error;
pub mod prover;
pub mod sequencer;
pub mod verifier;
pub mod zk_rollup;

use config::Config;
use error::HVMError;

pub struct OffchainLabs {
    config: Config,
    prover: prover::Prover,
    sequencer: sequencer::Sequencer,
    verifier: verifier::Verifier,
}

impl OffchainLabs {
    pub fn new(config: Config) -> Result<Self, HVMError> {
        let prover = prover::create_zk_snark_prover();
        let sequencer = sequencer::Sequencer::new(zk_rollup::State::default());
        let verifier = verifier::create_zk_snark_verifier();

        Ok(Self {
            config,
            prover,
            sequencer,
            verifier,
        })
    }

    pub fn process_transaction(&mut self, transaction: &[u8]) -> Result<bool, HVMError> {
        let proof = self.prover.generate_proof(transaction)?;
        let is_valid = self.verifier.verify_proof(&proof)?;
        
        if is_valid {
            self.sequencer.apply_proof(proof)?;
        }

        Ok(is_valid)
    }

    pub fn get_current_state(&self) -> Result<zk_rollup::State, HVMError> {
        Ok(self.sequencer.get_current_state())
    }
}