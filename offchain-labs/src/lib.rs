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
            prover,
            sequencer,
            verifier,
        })
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<bool, HVMError> {
        println!("Processing transaction: {:?}", transaction);
        self.sequencer.process_transaction(transaction)?;

        if let Some(batch) = self.sequencer.create_batch(true)? {
            println!("Batch created: {:?}", batch);
            let proof = self.prover.generate_proof(&batch).map_err(|e| {
                println!("Error generating proof: {:?}", e);
                e
            })?;
            println!("Proof generated: {:?}", proof);
            let is_valid = self.verifier.verify_proof(&proof).map_err(|e| {
                println!("Error verifying proof: {:?}", e);
                e
            })?;
            println!("Proof verification result: {}", is_valid);
            
            if is_valid {
                self.sequencer.apply_proof(proof, &batch)?;
                println!("Proof applied");
            }
    
            Ok(is_valid)
        } else {
            println!("No batch created");
            Ok(true)
        }
    }

    pub fn get_current_state(&self) -> Result<zk_rollup::State, HVMError> {
        Ok(self.sequencer.get_current_state())
    }

    pub fn pending_transactions_count(&self) -> usize {
        self.sequencer.pending_transactions_count()
    }

    pub fn processed_transactions_count(&self) -> usize {
        self.sequencer.processed_transactions_count()
    }

    pub fn get_pending_transactions(&self) -> &std::collections::VecDeque<Transaction> {
        self.sequencer.get_pending_transactions()
    }

    pub fn get_processed_transactions(&self) -> &Vec<Transaction> {
        self.sequencer.get_processed_transactions()
    }
}