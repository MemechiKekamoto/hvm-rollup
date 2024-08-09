use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;
use subxt_signer::sr25519::dev;
use warp::Filter;
use tracing::{info, error};
use clap::Parser;
use hex;
use url::Url;
use sp_core::{H256, Hasher, blake2_256};
use std::convert::Infallible;
use warp::reject::Reject;

#[subxt::subxt(runtime_metadata_path = "./artifacts/qf_metadata.scale")]
pub mod polkadot {}

#[derive(Debug, Deserialize, Serialize)]
struct ExecuteProgramData {
    program_id: H256,
    input: Vec<u8>,
}

#[derive(Debug, Clone, Parser)]
pub struct RelayOpts {
    #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
    pub substrate_url: Url,

    #[arg(short, long, default_value = "3030")]
    pub port: u16,
}

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Reject for CustomError {}

pub async fn run(opts: RelayOpts) -> Result<(), Box<dyn std::error::Error>> {
    let client = OnlineClient::<PolkadotConfig>::from_url(opts.substrate_url.as_str()).await?;

    let routes = warp::post()
        .and(warp::path("execute"))
        .and(warp::body::json())
        .and_then(move |data: ExecuteProgramData| {
            let client = client.clone();
            async move {
                match execute_bend_program(&client, data).await {
                    Ok(_) => Ok(warp::reply::json(&"Success")),
                    Err(e) => Err(warp::reject::custom(CustomError { message: e.to_string() })),
                }
            }
        });

    info!("Starting the relayer server on http://127.0.0.1:{}", opts.port);
    warp::serve(routes).run(([127, 0, 0, 1], opts.port)).await;
    Ok(())
}

async fn handle_request(opts: RelayOpts, program_data: ExecuteProgramData) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received data: {:?}", program_data);

    let calldata = execute_program_data_to_h256(&program_data);
    info!("Converted to H256: {:?}", calldata);

    match send_extrinsic(&opts, calldata).await {
        Ok(_) => Ok(warp::reply::json(&"Success")),
        Err(e) => {
            error!("Error sending to substrate: {:?}", e);
            Err(warp::reject::custom(CustomError { message: e.to_string() }))
        },
    }
}

fn execute_program_data_to_h256(data: &ExecuteProgramData) -> H256 {
    let mut input = data.program_id.as_bytes().to_vec();
    input.extend_from_slice(&data.input);
    H256::from(blake2_256(&input))
}

fn hash_to_hex_string(hash: &[u8; 32]) -> String {
    format!("0x{}", hex::encode(hash))
}

async fn send_extrinsic(opts: &RelayOpts, calldata: H256) -> Result<(), Box<dyn std::error::Error>> {
    let client = OnlineClient::<PolkadotConfig>::from_url(opts.substrate_url.as_str()).await?;
    let metadata = client.metadata();
    let types = metadata.types();
    let pallet = metadata
        .pallet_by_name("OffchainGateModule".as_ref())
        .ok_or_else(|| anyhow::anyhow!("pallet not found"))?;

    let from = dev::alice();

    let call = pallet
        .call_variant_by_name("verify_calldata")
        .ok_or_else(|| anyhow::anyhow!("function not found"))?;
    let pallet_index = pallet.index();
    let call_index = call.index;
    info!("Call: {:?}", call);
    info!("pallet_index: {:?}", pallet_index);
    info!("call_index: {:?}", call_index);

    let call_hash = pallet
        .call_hash("verify_calldata")
        .ok_or_else(|| anyhow::anyhow!("hash not found"))?;
    info!("call_hash: {:?}", hash_to_hex_string(&call_hash));

    let tx = polkadot::tx().offchain_gate_module().verify_calldata(calldata);

    let latest_block = client.blocks().at_latest().await?;
    let tx_params = Params::new()
        .tip(1_000)
        .mortal(latest_block.header(), 32)
        .build();
    let tx_hash = client.tx().sign_and_submit(&tx, &from, tx_params).await?;

    info!("Transaction submitted with hash: {:?}", tx_hash);
    Ok(())
}

async fn execute_bend_program(client: &OnlineClient<PolkadotConfig>, data: ExecuteProgramData) -> Result<()> {
    info!("Executing Bend program: {:?}", data.program_id);

    let from = dev::alice();
    let tx = polkadot::tx().bend_program_execution().execute_program(data.program_id, data.input);

    let latest_block = client.blocks().at_latest().await?;
    let tx_params = subxt::config::polkadot::PolkadotExtrinsicParamsBuilder::new()
        .tip(1_000)
        .mortal(latest_block.header(), 32)
        .build();

    let tx_hash = client.tx().sign_and_submit(&tx, &from, tx_params).await?;
    info!("Bend program execution submitted. Transaction hash: {:?}", tx_hash);
    
    Ok(())
}