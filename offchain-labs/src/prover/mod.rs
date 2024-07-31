use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;
use crate::bend::{BendProgram, BendCircuit};
use ark_bn254::Bn254;
use ark_groth16::{Groth16, ProvingKey};
use ark_snark::SNARK;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::thread_rng;

pub struct ZKProver {
    proving_key: ProvingKey<Bn254>,
}

impl ZKProver {
    pub fn new(proving_key: ProvingKey<Bn254>) -> Self {
        Self { proving_key }
    }

    pub fn generate_proof(&self, _batch: &Batch) -> Result<Proof, HVMError> {
        let circuit = BendCircuit::default();
        let mut rng = thread_rng();
        
        let proof = Groth16::<Bn254>::prove(&self.proving_key, circuit, &mut rng)
            .map_err(|e| HVMError::Prover(format!("Failed to generate proof: {}", e)))?;

        let mut proof_bytes = Vec::new();
        proof.serialize_uncompressed(&mut proof_bytes)
            .map_err(|e| HVMError::Prover(format!("Failed to serialize proof: {}", e)))?;
        
        Ok(Proof::new(proof_bytes))
    }

    pub fn estimate_resource_usage(&self, _program: &BendProgram) -> Result<ResourceUsage, HVMError> {
        Ok(ResourceUsage { cpu_cycles: 1000, memory_usage: 2048 })
    }

    pub fn optimize_program(&self, program: &BendProgram) -> Result<BendProgram, HVMError> {
        Ok(program.clone())
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