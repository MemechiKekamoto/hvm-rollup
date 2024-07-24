use crate::error::HVMError;
use crate::zk_rollup::Proof;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, PreparedVerifyingKey, VerifyingKey};
use ark_snark::SNARK;
use ark_serialize::CanonicalDeserialize;
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use std::ops::AddAssign;

pub struct ZKVerifier {
    verifying_key: PreparedVerifyingKey<Bn254>,
}

impl ZKVerifier {
    pub fn new(verifying_key: VerifyingKey<Bn254>) -> Self {
        println!("Creating new ZKVerifier with verifying key: {:?}", verifying_key);
        let prepared_verifying_key = Groth16::<Bn254>::process_vk(&verifying_key).unwrap();
        Self { verifying_key: prepared_verifying_key }
    }

    pub fn verify_proof(&self, proof: &Proof, public_inputs: &[Fr]) -> Result<bool, HVMError> {
        println!("Public input length: {:?}", public_inputs.len());
        println!("Verifying with processed key length: {:?}", self.verifying_key.vk.gamma_abc_g1.len());

        if (public_inputs.len() + 1) != self.verifying_key.vk.gamma_abc_g1.len() {
            return Err(HVMError::Verifier("Malformed verifying key".to_string()));
        }

        println!("Verifying proof: {:?}", proof);
        let groth16_proof = ark_groth16::Proof::<Bn254>::deserialize_uncompressed(&proof.data[..])
            .map_err(|e| HVMError::Verifier(format!("Failed to deserialize proof: {}", e)))?;
        
        let pvk = &self.verifying_key;
        let mut g_ic = pvk.vk.gamma_abc_g1[0].into_group();
        for (i, b) in public_inputs.iter().zip(pvk.vk.gamma_abc_g1.iter().skip(1)) {
            g_ic.add_assign(&b.mul_bigint(i.into_bigint()));
        }
        
        let result = Groth16::<Bn254>::verify_with_processed_vk(pvk, &public_inputs, &groth16_proof)
            .map_err(|e| HVMError::Verifier(format!("Proof verification failed: {}", e)));
        println!("Verification result: {:?}", result);

        result.map_err(|e| HVMError::Verifier(format!("Proof verification failed: {}", e)))
    }
}

pub fn create_zk_verifier(verifying_key: VerifyingKey<Bn254>) -> ZKVerifier {
    ZKVerifier::new(verifying_key)
}