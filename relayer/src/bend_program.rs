use crate::{BendProgram, BendProgramMetadata};
use anyhow::Result;
use sp_core::H256;
use subxt::{OnlineClient, PolkadotConfig};

pub async fn get_bend_program(client: &OnlineClient<PolkadotConfig>, id: H256) -> Result<BendProgram> {
    let program = BendProgram {
        id,
        code: vec![0, 1, 2, 3],
        metadata: BendProgramMetadata {
            name: "Test Program".to_string(),
            version: "1.0.0".to_string(),
            description: "Bend program for test".to_string(),
            author: "Author".to_string(),
        },
    };
    Ok(program)
}

pub fn validate_bend_program(program: &BendProgram) -> Result<()> {
    if program.code.is_empty() {
        anyhow::bail!("Invalid Bend program: empty code");
    }
    Ok(())
}