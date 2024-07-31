use crate::error::HVMError;
use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_relations::lc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

pub mod storage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BendProgram {
    id: String,
    bytecode: Vec<u8>,
    metadata: ProgramMetadata,
    author: String,
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

    pub fn execute(&self, _inputs: Vec<u8>) -> Result<Vec<Fr>, HVMError> {
        Ok(vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)])
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

pub struct BendCircuit;

impl Default for BendCircuit {
    fn default() -> Self {
        Self
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