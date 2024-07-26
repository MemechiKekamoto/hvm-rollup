use crate::error::HVMError;
use ark_bn254::Fr;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BendProgram {
    bytecode: Vec<u8>,
    metadata: ProgramMetadata,
    author: String,
    execution_cost: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl BendProgram {
    pub fn new(bytecode: Vec<u8>, metadata: ProgramMetadata, author: String, execution_cost: u64) -> Self {
        Self { bytecode, metadata, author, execution_cost }
    }

    pub fn execute_and_trace(&self) -> Result<Vec<Fr>, HVMError> {
        Ok(vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)])
    }

    pub fn get_public_inputs(&self) -> Vec<Fr> {
        vec![Fr::from(1u64)]
    }

    pub fn execution_cost(&self) -> u64 {
        self.execution_cost
    }
}

pub struct BendVM {}

impl BendVM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn estimate_resources(&self, program: &BendProgram) -> Result<crate::prover::ResourceUsage, HVMError> {
        Ok(crate::prover::ResourceUsage {
            cpu_cycles: program.execution_cost(),
            memory_usage: 1024,
        })
    }

    pub fn optimize(&self, program: &BendProgram) -> Result<BendProgram, HVMError> {
        Ok(program.clone())
    }

    pub fn execute(&self, _program: &BendProgram, _inputs: &[Fr]) -> Result<Vec<Fr>, HVMError> {
        Ok(vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)])
    }
}