use sp_core::{H256, U256};
use sp_runtime::traits::Hash;

fn u256_to_le_bytes(value: U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    value.to_little_endian(&mut bytes);
    bytes
}

pub fn verify_calldata(data: &[u8], proof: &[u8], _nonce: U256) -> bool {
    if proof.len() < 32 {
        return false;
    }

    let hash = sp_runtime::traits::BlakeTwo256::hash(data);
    let hash_bytes = hash.as_ref();

    hash_bytes.starts_with(&proof[..8])
}

pub fn generate_tx_hash(data: &[u8], proof: &[u8], nonce: U256) -> H256 {
    let mut input = data.to_vec();
    input.extend_from_slice(proof);
    input.extend_from_slice(&u256_to_le_bytes(nonce));

    sp_runtime::traits::BlakeTwo256::hash(&input)
}