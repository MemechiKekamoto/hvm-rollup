use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;
use crate::prover::ProverLibs;
use bellman::{
    Circuit, ConstraintSystem, SynthesisError,
    groth16::{
        create_random_proof, generate_random_parameters,
        Parameters, Proof as BMProof,
    },
};
use bls12_381::{Bls12, Scalar};
use ff::PrimeField;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

pub struct ZKSnarkLibs {
    proving_key: Parameters<Bls12>,
}

#[derive(Serialize, Deserialize)]
struct SerializableProof(#[serde(with = "proof_serde")] BMProof<Bls12>);

mod proof_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(proof: &BMProof<Bls12>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = Vec::new();
        proof.write(&mut bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BMProof<Bls12>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        BMProof::read(&bytes[..]).map_err(serde::de::Error::custom)
    }
}

impl ZKSnarkLibs {
    pub fn new() -> Self {
        let circuit = DummyCircuit { x: None };
        let params = generate_random_parameters::<Bls12, _, _>(circuit, &mut thread_rng()).unwrap();
        Self { proving_key: params }
    }
}

impl ProverLibs for ZKSnarkLibs {
    fn generate_proof(&self, batch: &Batch) -> Result<Proof, HVMError> {
        let circuit = DummyCircuit { x: Some(Scalar::from(batch.transactions().len() as u64)) };
        
        let proof = create_random_proof(circuit, &self.proving_key, &mut thread_rng())
            .map_err(|e| HVMError::Prover(format!("Failed to generate proof: {}", e)))?;
        
        let serializable_proof = SerializableProof(proof);
        let proof_bytes = bincode::serialize(&serializable_proof)
            .map_err(|e| HVMError::Prover(format!("Failed to serialize proof: {}", e)))?;
        
        Ok(Proof::new(proof_bytes))
    }
}

struct DummyCircuit {
    x: Option<Scalar>,
}

impl Circuit<Scalar> for DummyCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let x = cs.alloc(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        cs.enforce(|| "x * x = x", |lc| lc + x, |lc| lc + x, |lc| lc + x);
        Ok(())
    }
}