pub mod config;
pub mod error;
pub mod prover;
pub mod sequencer;
pub mod verifier;
pub mod zk_rollup;
pub mod bend;

pub use config::Config;
use std::collections::HashMap;
use error::HVMError;
use sequencer::Transaction;
use prover::ZKProver;
use verifier::ZKVerifier;
use bend::{BendProgram, storage::Storage};

use ark_bn254::Bn254;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_snark::SNARK;

pub struct OffchainLabs {
    prover: ZKProver,
    sequencer: sequencer::Sequencer,
    verifier: ZKVerifier,
    storage: Storage,
    user_balances: HashMap<String, u64>,
}

impl OffchainLabs {
    pub fn new(config: Config) -> Result<Self, HVMError> {        
        let (pk, vk) = Self::generate_zk_keys(&config)?;
        
        let prover = ZKProver::new(pk);
        let sequencer = sequencer::Sequencer::new(zk_rollup::State::default(), config.sequencer_config.clone());
        let verifier = ZKVerifier::new(vk);
        let storage = Storage::new();
        let user_balances = HashMap::new();

        Ok(Self {
            prover,
            sequencer,
            verifier,
            storage,
            user_balances,
        })
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result<bool, HVMError> {
        self.sequencer.process_transaction(transaction)?;
    
        if let Some(batch) = self.sequencer.create_batch(true)? {
            let proof = self.prover.generate_proof(&batch)?;
            let is_valid = self.verifier.verify_proof(&proof, &batch.programs().iter().flat_map(|p| p.get_public_inputs()).collect::<Vec<_>>())?;
            
            if is_valid {
                self.sequencer.apply_proof(proof, &batch)?;
            }
    
            Ok(is_valid)
        } else {
            Ok(true)
        }
    }

    pub fn submit_program(&mut self, program: BendProgram) -> Result<(), HVMError> {
        self.sequencer.submit_program(program.clone())?;
        self.storage.store_program(program)
    }

    pub fn deploy_program(&mut self, program: BendProgram) -> Result<(), HVMError> {
        self.sequencer.deploy_program(program.clone())?;
        self.storage.store_program(program)
    }

    pub fn execute_program(&mut self, program_id: &str, inputs: Vec<u8>, user_id: &str) -> Result<Vec<u8>, HVMError> {
        let program = self.storage.load_program(program_id)?;
        let estimated_usage = self.prover.estimate_resource_usage(&program)?;
        let required_balance = self.calculate_required_balance(&estimated_usage);

        self.check_and_deduct_balance(user_id, required_balance)?;

        let result = self.sequencer.execute_program(program_id, inputs)?;

        let actual_usage = self.prover.estimate_resource_usage(&program)?;
        let actual_cost = self.calculate_actual_cost(&actual_usage);
        self.refund_excess_balance(user_id, required_balance, actual_cost);

        Ok(result)
    }

    fn calculate_required_balance(&self, usage: &prover::ResourceUsage) -> u64 {
        usage.cpu_cycles + usage.memory_usage
    }

    fn check_and_deduct_balance(&mut self, user_id: &str, amount: u64) -> Result<(), HVMError> {
        let balance = self.user_balances.entry(user_id.to_string()).or_insert(0);
        if *balance < amount {
            return Err(HVMError::InsufficientBalance());
        }
        *balance -= amount;
        Ok(())
    }

    fn calculate_actual_cost(&self, usage: &prover::ResourceUsage) -> u64 {
        usage.cpu_cycles + usage.memory_usage
    }

    fn refund_excess_balance(&mut self, user_id: &str, deducted: u64, actual_cost: u64) {
        if actual_cost < deducted {
            let refund = deducted - actual_cost;
            *self.user_balances.entry(user_id.to_string()).or_insert(0) += refund;
        }
    }

    pub fn deposit_funds(&mut self, user_id: &str, amount: u64) {
        *self.user_balances.entry(user_id.to_string()).or_insert(0) += amount;
    }

    pub fn get_balance(&self, user_id: &str) -> u64 {
        *self.user_balances.get(user_id).unwrap_or(&0)
    }

    fn generate_zk_keys(_config: &Config) -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), HVMError> {
        let circuit = bend::BendCircuit::default();
        let mut rng = ark_std::rand::thread_rng();
        Groth16::<Bn254>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| HVMError::Setup(format!("Failed to generate ZK-SNARK keys: {}", e)))
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

    pub fn estimate_program_resources(&self, program: &BendProgram) -> Result<prover::ResourceUsage, HVMError> {
        self.prover.estimate_resource_usage(program)
    }

    pub fn optimize_program(&self, program: &BendProgram) -> Result<BendProgram, HVMError> {
        self.prover.optimize_program(program)
    }
}