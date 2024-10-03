use crate::error::HVMError;
use crate::zk_rollup::Proof;
use crate::sequencer::Batch;
use crate::bend::{BendProgram, BendVM};
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, ProvingKey};
use ark_snark::SNARK;
use ark_serialize::CanonicalSerialize;
use ark_std::rand::thread_rng;
use ark_relations::lc;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::One;

#[derive(Clone)]
pub struct ZKProver {
    pub proving_key: ProvingKey<Bn254>,
    bend_vm: BendVM,
}

impl ZKProver {
    pub fn new(proving_key: ProvingKey<Bn254>) -> Self {
        Self {
            proving_key,
            bend_vm: BendVM::new(),
        }
    }

    pub fn generate_proof(&self, batch: &Batch) -> Result<Proof, HVMError> {
        println!("Generating proof for batch: {:?}", batch);
        let circuit = BendProgramCircuit::new(batch);
        
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

    pub fn estimate_resource_usage(&self, program: &BendProgram) -> Result<ResourceUsage, HVMError> {
        self.bend_vm.estimate_resources(program)
    }

    pub fn optimize_program(&self, program: &BendProgram) -> Result<BendProgram, HVMError> {
        self.bend_vm.optimize(program)
    }
}

pub struct BendProgramCircuit {
    execution_trace: Vec<Fr>,
}

impl BendProgramCircuit {
    pub fn new(batch: &Batch) -> Self {
        let mut execution_trace = Vec::new();
        for program in batch.programs() {
            execution_trace.extend(program.execute_and_trace().unwrap_or_default());
        }
        Self {
            execution_trace,
        }
    }
}

impl ConstraintSynthesizer<Fr> for BendProgramCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        for (i, &value) in self.execution_trace.iter().enumerate() {
            let var = cs.new_witness_variable(|| Ok(value))?;
            if i > 0 {
                let prev_var = cs.new_witness_variable(|| Ok(self.execution_trace[i - 1]))?;
                cs.enforce_constraint(lc!() + prev_var, lc!() + (Fr::one(), lc!()), lc!() + var)?;
            }
        }
        Ok(())
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