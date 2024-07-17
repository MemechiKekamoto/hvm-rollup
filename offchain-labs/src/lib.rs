pub mod config;
pub mod error;
pub mod prover;
pub mod sequencer;
pub mod verifier;
pub mod zk_rollup;

pub use config::Config;
use error::HVMError;
use sequencer::Transaction;

pub struct OffchainLabs {
    #[allow(dead_code)]
    config: Config,
    prover: prover::Prover,
    sequencer: sequencer::Sequencer,
    verifier: verifier::Verifier,
}

impl OffchainLabs {
    pub fn new(config: Config) -> Result<Self, HVMError> {
        let prover = prover::create_zk_snark_prover();
        let sequencer = sequencer::Sequencer::new(zk_rollup::State::default(), config.sequencer_config.clone());
        let verifier = verifier::create_zk_snark_verifier();

        Ok(Self {
            config,
            prover,
            sequencer,
            verifier,
        })
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<bool, HVMError> {
        self.sequencer.process_transaction(transaction)?;

        if let Some(batch) = self.sequencer.create_batch()? {
            let proof = self.prover.generate_proof(&batch)?;
            let is_valid = self.verifier.verify_proof(&proof)?;
            
            if is_valid {
                self.sequencer.apply_proof(proof)?;
            }

            Ok(is_valid)
        } else {
            Ok(true)
        }
    }

    pub fn get_current_state(&self) -> Result<zk_rollup::State, HVMError> {
        Ok(self.sequencer.get_current_state())
    }

    pub fn pending_transactions_count(&self) -> usize {
        self.sequencer.pending_transactions_count()
    }
}