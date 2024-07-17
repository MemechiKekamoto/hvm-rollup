use crate::error::HVMError;
use crate::zk_rollup::Proof;
use super::super::VerifierLibs;
use bellman::{
    Circuit, ConstraintSystem, SynthesisError,
    groth16::{
        generate_random_parameters, prepare_verifying_key, verify_proof,
        Proof as BMProof, VerifyingKey,
    },
};
use bls12_381::Bls12;
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
    pub fn new() -> Self {
        let circuit = DummyCircuit { x: None };
        let params = generate_random_parameters::<Bls12, _, _>(circuit, &mut rand::thread_rng()).unwrap();
        Self { verification_key: params.vk }
    }
}

impl VerifierLibs for ZKSnarkLibs {
    fn verify_proof(&self, proof: &Proof) -> Result<bool, HVMError> {
        let serializable_proof: SerializableProof = bincode::deserialize(&proof.data)
            .map_err(|e| HVMError::Verifier(format!("Failed to deserialize proof: {}", e)))?;
        
        let groth16_proof = serializable_proof.0;
        
        let public_inputs = vec![bls12_381::Scalar::zero()];
        
        let pvk = prepare_verifying_key(&self.verification_key);
        
        match verify_proof(&pvk, &groth16_proof, &public_inputs) {
            Ok(()) => Ok(true),
            Err(e) => Err(HVMError::Verifier(format!("Proof verification failed: {}", e)))
        }
    }
}

struct DummyCircuit {
    x: Option<bls12_381::Scalar>,
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