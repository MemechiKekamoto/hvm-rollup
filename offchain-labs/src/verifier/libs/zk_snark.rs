use crate::error::HVMError;
use crate::zk_rollup::Proof;
use super::super::VerifierLibs;
use bellman::{
    Circuit, ConstraintSystem, SynthesisError,
    groth16::{
        prepare_verifying_key, verify_proof,
        Proof as BMProof, VerifyingKey,
    },
};
use bls12_381::{Bls12, Scalar};
use ff::PrimeField;
use serde::{Serialize, Deserialize};

pub struct ZKSnarkLibs {
    verification_key: VerifyingKey<Bls12>,
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
    pub fn new(verification_key: VerifyingKey<Bls12>) -> Self {
        Self { verification_key }
    }
}

impl VerifierLibs for ZKSnarkLibs {
    fn verify_proof(&self, proof: &Proof) -> Result<bool, HVMError> {
        let serializable_proof: SerializableProof = bincode::deserialize(&proof.data)
            .map_err(|e| HVMError::Verifier(format!("Failed to deserialize proof: {}", e)))?;
        
        let groth16_proof = serializable_proof.0;
        
        let public_inputs = vec![Scalar::zero()];
        
        let pvk = prepare_verifying_key(&self.verification_key);
        
        verify_proof(&pvk, &groth16_proof, &public_inputs)
            .map_err(|e| HVMError::Verifier(format!("Proof verification failed: {}", e)))
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