pub mod config;
pub mod error;
pub mod prover;
pub mod sequencer;
pub mod verifier;
pub mod zk_rollup;

pub use config::Config;
use error::HVMError;
use sequencer::Transaction;
use prover::{BatchCircuit, ZKProver};
use verifier::ZKVerifier;

use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::rand::thread_rng;

pub struct OffchainLabs {
    prover: ZKProver,
    sequencer: sequencer::Sequencer,
    verifier: ZKVerifier,
    public_inputs: Vec<Fr>,
}

impl OffchainLabs {
    pub fn new(config: Config) -> Result<Self, HVMError> {
        let mut rng = thread_rng();
        let circuit = BatchCircuit::<Fr>::new(&sequencer::Batch::new(vec![]));
        
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| HVMError::Setup(format!("Failed to generate ZK-SNARK keys: {}", e)))?;
        
        let prover = ZKProver::new(pk);
        let sequencer = sequencer::Sequencer::new(zk_rollup::State::default(), config.sequencer_config.clone());
        let verifier = ZKVerifier::new(vk.clone());

        let public_inputs = vec![Fr::from(1u64); vk.gamma_abc_g1.len() - 1];

        Ok(Self {
            prover,
            sequencer,
            verifier,
            public_inputs,
        })
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<bool, HVMError> {
        println!("Processing transaction: {:?}", transaction);
        self.sequencer.process_transaction(transaction)?;
    
        if let Some(batch) = self.sequencer.create_batch(true)? {
            println!("Batch created: {:?}", batch);
            let proof = self.prover.generate_proof(&batch)?;
            println!("Proof generated: {:?}", proof);           

            let is_valid = self.verifier.verify_proof(&proof, &self.public_inputs)?;
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