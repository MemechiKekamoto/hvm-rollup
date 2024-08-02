use crate::error::HVMError;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_relations::lc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use wasmer::{Store, Module, Instance, Value, imports, Memory};
use log::{error, debug};

pub mod storage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BendProgram {
    pub id: String,
    pub bytecode: Vec<u8>,
    pub metadata: ProgramMetadata,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl BendProgram {
    pub fn new(bytecode: Vec<u8>, metadata: ProgramMetadata, author: String) -> Self {
        let id = Self::generate_id(&bytecode);
        Self { id, bytecode, metadata, author }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn execute(&self, inputs: Vec<u8>) -> Result<Vec<Fr>, HVMError> {
        let mut store = Store::default();
        let module = Module::new(&store, &self.bytecode)
            .map_err(|e| HVMError::Execution(format!("Failed to create module: {}", e)))?;
        let import_object = imports! {};
        let instance = Instance::new(&mut store, &module, &import_object)
            .map_err(|e| HVMError::Execution(format!("Failed to instantiate module: {}", e)))?;
    
        let memory = instance.exports.get_memory("memory")
            .map_err(|e| HVMError::Execution(format!("Module does not export memory: {}", e)))?;
    
        self.write_inputs_to_memory(&store, &memory, &inputs)?;
    
        let run = instance.exports.get_function("run")
            .map_err(|e| HVMError::Execution(format!("Failed to get run function: {}", e)))?;
    
        debug!("Executing WebAssembly module");
        let result = run.call(&mut store, &[]);
        match result {
            Ok(output) => {
                debug!("WebAssembly execution successful");
                self.read_outputs_from_memory(&store, &memory, &output)
            },
            Err(e) => {
                error!("WebAssembly execution failed: {}", e);
                Err(HVMError::Execution(format!("Failed to execute program: {}", e)))
            }
        }
    }

    fn write_inputs_to_memory(&self, store: &Store, memory: &Memory, inputs: &[u8]) -> Result<(), HVMError> {
        let mem_view = memory.view(store);
        mem_view.write(0, inputs)
            .map_err(|e| HVMError::Execution(format!("Failed to write inputs to memory: {}", e)))?;
        Ok(())
    }

    fn read_outputs_from_memory(&self, store: &Store, memory: &Memory, results: &[Value]) -> Result<Vec<Fr>, HVMError> {
        let ptr = results[0].i32().unwrap() as u64;
        let len = results[1].i32().unwrap() as u64;
        let mem_view = memory.view(store);
        let mut output_bytes = vec![0u8; len as usize];
        mem_view.read(ptr, &mut output_bytes)
            .map_err(|e| HVMError::Execution(format!("Failed to read outputs from memory: {}", e)))?;
    
        let field_elements = output_bytes.chunks_exact(32)
            .map(|chunk| Fr::from_le_bytes_mod_order(chunk))
            .collect::<Vec<_>>();
    
        Ok(field_elements)
    }

    pub fn get_public_inputs(&self) -> Vec<Fr> {
        vec![Fr::from(1u64)]
    }

    fn generate_id(bytecode: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytecode);
        format!("{:x}", hasher.finalize())
    }
}

pub struct BendCircuit {
    pub inputs: Vec<Fr>,
    pub outputs: Vec<Fr>,
}

impl Default for BendCircuit {
    fn default() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl ConstraintSynthesizer<Fr> for BendCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| Ok(Fr::from(10u64)))?;
        let b = cs.new_witness_variable(|| Ok(Fr::from(20u64)))?;
        let c = cs.new_input_variable(|| Ok(Fr::from(30u64)))?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}