use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, ProvingKey};
use ark_snark::SNARK;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::thread_rng;
use ark_relations::lc;

pub struct ZKProver {
    proving_key: ProvingKey<Bn254>,
}

impl ZKProver {
    pub fn new(proving_key: ProvingKey<Bn254>) -> Self {
        Self { proving_key }
    }

    pub fn generate_proof(&self, batch: &Batch) -> Result<Proof, HVMError> {
        println!("Generating proof for batch: {:?}", batch);
        let circuit = BatchCircuit::<Fr>::new(batch);
        
        let mut rng = thread_rng();
        println!("Proving key: {:?}", self.proving_key);
        let proof = Groth16::<Bn254>::prove(&self.proving_key, circuit, &mut rng)
            .map_err(|e| HVMError::Prover(format!("Failed to generate proof: {}", e)))?;

        println!("Generated Groth16 proof: {:?}", proof);
        
        let mut proof_bytes = Vec::new();
        proof.serialize_uncompressed(&mut proof_bytes)
            .map_err(|e| HVMError::Prover(format!("Failed to serialize proof: {}", e)))?;

        println!("Serialized proof bytes: {:?}", proof_bytes);
        
        Ok(Proof::new(proof_bytes))
    }
}

pub fn create_zk_prover(proving_key: ProvingKey<Bn254>) -> ZKProver {
    ZKProver::new(proving_key)
}

use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_ff::Field;

pub struct BatchCircuit<F: Field> {
    transactions: Vec<(F, F, F)>,
}

impl<F: Field> BatchCircuit<F> {
    pub fn new(batch: &Batch) -> Self {
        let transactions = batch
            .transactions()
            .iter()
            .map(|tx| {
                (
                    F::from(tx.amount as u64),
                    F::from(tx.nonce as u64),
                    F::from(1u64),
                )
            })
            .collect();
        
        Self { transactions }
    }
}

impl<F: Field> ConstraintSynthesizer<F> for BatchCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        for (_i, (amount, nonce, _)) in self.transactions.iter().enumerate() {
            let amount_var = cs.new_witness_variable(|| Ok(*amount))?;
            let nonce_var = cs.new_witness_variable(|| Ok(*nonce))?;
            
            let product_var = cs.new_witness_variable(|| {
                let product = *amount * *nonce;
                Ok(product)
            })?;

            cs.enforce_constraint(
                lc!() + amount_var,
                lc!() + nonce_var,
                lc!() + product_var
            )?;
        }
        
        Ok(())
    }
}