pub mod config;
pub mod error;
pub mod prover;
pub mod sequencer;
pub mod verifier;
pub mod zk_rollup;
pub mod bend;

pub use config::Config;
use error::HVMError;
use sequencer::Transaction;
use prover::{BendProgramCircuit, ZKProver};
use verifier::ZKVerifier;
use bend::BendProgram;

use ark_bn254::Bn254;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use zk_rollup::Proof;

#[derive(Clone)]
pub struct OffchainLabs {
    prover: ZKProver,
    sequencer: sequencer::Sequencer,
    verifier: ZKVerifier,
}

impl OffchainLabs {
    pub fn new(config: Config) -> Result<Self, HVMError> {
        let mut rng = thread_rng();
        let program = BendProgram::new(
            vec![],
            bend::ProgramMetadata {
                name: "Bend program".to_string(),
                version: "0.1.0".to_string(),
                description: "Program for setup".to_string(),
            },
            "Alice".to_string(),
            1000,
        );

        let binding = sequencer::Batch::new(vec![], vec![program.clone()]);
        let circuit = BendProgramCircuit::new(&binding);
        
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| HVMError::Setup(format!("Failed to generate ZK-SNARK keys: {}", e)))?;
        
        let prover = ZKProver::new(pk);
        let sequencer = sequencer::Sequencer::new(zk_rollup::State::default(), config.sequencer_config.clone());
        let verifier = ZKVerifier::new(vk.clone());

        Ok(Self {
            prover,
            sequencer,
            verifier,
        })
    }

    pub fn process_transaction_ex(&mut self, transaction: Transaction) -> Result<(bool, Option<Proof>), HVMError> {
        println!("Processing transaction: {:?}", transaction);
        self.sequencer.process_transaction(transaction)?;
    
        if let Some(batch) = self.sequencer.create_batch(true)? {
            println!("Batch created: {:?}", batch);
            let proof = self.prover.generate_proof(&batch)?;
            println!("Proof generated: {:?}", proof);           

            let is_valid = self.verifier.verify_proof(&proof, &batch.programs().iter().flat_map(|p| p.get_public_inputs()).collect::<Vec<_>>())?;
            println!("Proof verification result: {}", is_valid);
            
            if is_valid {
                self.sequencer.apply_proof(proof.clone(), &batch)?;
                println!("Proof applied");
            }
    
            Ok((is_valid, Some(proof.clone())))
        } else {
            println!("No batch created");
            Ok((true, None))
        }
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<bool, HVMError> {
        println!("Processing transaction: {:?}", transaction);
        self.sequencer.process_transaction(transaction)?;
    
        if let Some(batch) = self.sequencer.create_batch(true)? {
            println!("Batch created: {:?}", batch);
            let proof = self.prover.generate_proof(&batch)?;
            println!("Proof generated: {:?}", proof);           

            let is_valid = self.verifier.verify_proof(&proof, &batch.programs().iter().flat_map(|p| p.get_public_inputs()).collect::<Vec<_>>())?;
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

    pub fn submit_program(&mut self, program: BendProgram) -> Result<(), HVMError> {
        self.sequencer.submit_program(program)
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