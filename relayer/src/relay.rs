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
use sp_core::H256;
use std::convert::Infallible;
use warp::reject::Reject;

#[subxt::subxt(runtime_metadata_path = "./artifacts/qf_metadata.scale")]
pub mod polkadot {}

#[derive(Debug, Deserialize, Serialize)]
struct DummyData {
    data: String,
}

#[derive(Debug, Clone, Parser)]
pub struct RelayOpts {
    #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
    pub substrate_url: Url,

    // #[arg(short, long)]
    // pub sequencer_url: Url,

    #[arg(short = 't', long, default_value = "0", value_name = "NUM")]
    pub threads: usize,

    #[arg(short, long, default_value = "100")]
    pub batch_size: usize,
}

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Reject for CustomError {}

pub async fn run(opts: RelayOpts) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let routes = warp::path("receive")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |dummy_data: DummyData| handle_request(opts.clone(), dummy_data));

    info!("Starting the relayer server on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

async fn handle_request(opts: RelayOpts, dummy_data: DummyData) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received data: {:?}", dummy_data);

    let calldata = convert_to_calldata(&dummy_data.data);
    info!("Calldata: {:?}", calldata);
    match send_extrinsic(&opts, calldata).await {
        Ok(_) => Ok(warp::reply::json(&"Success")),
        Err(e) => {
            error!("Error sending to substrate: {:?}", e);
            Err(warp::reject::custom(CustomError { message: e.to_string() }))
        },
    }
}

fn convert_to_calldata(data: &str) -> H256 {
    let mut padded_data = [0u8; 32];
    let bytes = data.as_bytes();

    let len = bytes.len().min(32);
    padded_data[..len].copy_from_slice(&bytes[..len]);

    H256::from_slice(&padded_data)
}

fn hash_to_hex_string(hash: &[u8; 32]) -> String {
    format!("0x{}", hex::encode(hash))
}

async fn send_extrinsic(opts: &RelayOpts, calldata: H256) -> Result<(), Box<dyn std::error::Error>> {
    let client = OnlineClient::<PolkadotConfig>::from_url(opts.substrate_url.as_str()).await?;
    // let update_client = client.subscribe_to_updates();
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
