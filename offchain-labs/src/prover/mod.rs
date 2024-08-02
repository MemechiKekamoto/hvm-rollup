use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;
use crate::Transaction;
use crate::bend::{BendProgram, BendCircuit};
use ark_bn254::Bn254;
use ark_groth16::{Groth16, ProvingKey};
use ark_snark::SNARK;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::thread_rng;
use wasmer::{Store, Module, Instance, Cranelift};
use std::time::Instant;
use std::collections::HashMap;

pub struct ZKProver {
    proving_key: ProvingKey<Bn254>,
    program_cache: HashMap<String, BendProgram>,
}

impl ZKProver {
    pub fn new(proving_key: ProvingKey<Bn254>) -> Self {
        Self {
            proving_key,
            program_cache: HashMap::new(),
        }
    }

    pub fn generate_proof(&self, batch: &Batch) -> Result<Proof, HVMError> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for transaction in batch.transactions() {
            let program = self.get_program_for_transaction(transaction)?;
            let execution_result = program.execute(transaction.amount.clone())?;
            inputs.extend(program.get_public_inputs());
            outputs.extend(execution_result);
        }

        let circuit = BendCircuit {
            inputs,
            outputs,
        };
        let mut rng = thread_rng();
        
        let proof = Groth16::<Bn254>::prove(&self.proving_key, circuit, &mut rng)
            .map_err(|e| HVMError::Prover(format!("Failed to generate proof: {}", e)))?;

        let mut proof_bytes = Vec::new();
        proof.serialize_uncompressed(&mut proof_bytes)
            .map_err(|e| HVMError::Prover(format!("Failed to serialize proof: {}", e)))?;
        
        Ok(Proof::new(proof_bytes))
    }

    fn get_program_for_transaction(&self, transaction: &Transaction) -> Result<&BendProgram, HVMError> {
        let program_id = &transaction.program_id;
        println!("Program Id: {:?}", program_id);

        self.program_cache.get(program_id)
            .ok_or_else(|| HVMError::Prover(format!("Program not found for ID: {}", program_id)))
    }

    pub fn estimate_resource_usage(&self, program: &BendProgram) -> Result<ResourceUsage, HVMError> {
        let mut store = Store::new(Cranelift::default());
        let module = Module::new(&mut store, &program.bytecode)
            .map_err(|e| HVMError::Estimation(format!("Failed to create module: {}", e)))?;
        let import_object = wasmer::imports! {};
        let instance = Instance::new(&mut store, &module, &import_object)
            .map_err(|e| HVMError::Estimation(format!("Failed to instantiate module: {}", e)))?;

        let memory = instance.exports.get_memory("memory")
            .map_err(|e| HVMError::Estimation(format!("Failed to get memory: {}", e)))?;

        let run = instance.exports.get_function("run")
            .map_err(|e| HVMError::Estimation(format!("Failed to get run function: {}", e)))?;

        let start_memory = memory.view(&store).data_size() as u64;
        let start_time = Instant::now();

        run.call(&mut store, &[])
            .map_err(|e| HVMError::Estimation(format!("Failed to execute program: {}", e)))?;

        let end_time = Instant::now();
        let end_memory = memory.view(&store).data_size() as u64;

        Ok(ResourceUsage {
            cpu_cycles: end_time.duration_since(start_time).as_micros() as u64,
            memory_usage: end_memory - start_memory,
        })
    }

    pub fn optimize_program(&self, program: &BendProgram) -> Result<BendProgram, HVMError> {
        let optimized_bytecode = program.bytecode.clone();

        Ok(BendProgram::new(
            optimized_bytecode,
            program.metadata.clone(),
            program.author.clone()
        ))
    }

    pub fn add_program(&mut self, program: BendProgram) {
        self.program_cache.insert(program.id().to_string(), program);
    }
}

#[derive(Debug)]
pub struct ResourceUsage {
    pub cpu_cycles: u64,
    pub memory_usage: u64,
}

pub fn create_zk_prover(proving_key: ProvingKey<Bn254>) -> ZKProver {
    ZKProver::new(proving_key)
}