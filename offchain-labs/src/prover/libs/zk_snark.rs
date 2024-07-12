use crate::error::HVMError;
use crate::zk_rollup::Proof;
use super::super::ProverLibs;
use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::Bls12;
use rand::thread_rng;

pub struct ZKSnarkLibs {
    proving_key: groth16::ProvingKey<Bls12>,
}

impl ZKSnarkLibs {
    pub fn new() -> Self {
        let circuit = DummyCircuit { x: None };
        let params = groth16::generate_random_parameters::<Bls12, _, _>(circuit, &mut thread_rng()).unwrap();
        Self { proving_key: params }
    }
}

impl ProverLibs for ZKSnarkLibs {
    fn generate_proof(&self, input: &[u8]) -> Result<Proof, HVMError> {
        let circuit = DummyCircuit { x: Some(input[0] as u64) };
        
        let proof = groth16::create_random_proof(circuit, &self.proving_key, &mut thread_rng())
            .map_err(|e| HVMError::Prover(format!("Failed to generate proof: {}", e)))?;
        
        let proof_bytes = bincode::serialize(&proof)
            .map_err(|e| HVMError::Prover(format!("Failed to serialize proof: {}", e)))?;
        
        Ok(Proof::new(proof_bytes))
    }
}

struct DummyCircuit {
    x: Option<u64>,
}

impl Circuit<bls12_381::Scalar> for DummyCircuit {
    fn synthesize<CS: ConstraintSystem<bls12_381::Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let x = cs.alloc(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        cs.enforce(|| "x * x = x", |lc| lc + x, |lc| lc + x, |lc| lc + x);
        Ok(())
    }
}