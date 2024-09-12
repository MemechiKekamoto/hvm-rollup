use anyhow::Result;
use clap::Parser;
use log::{info, error};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use url::Url;

#[subxt::subxt(runtime_metadata_path = "./artifacts/qf_metadata.scale")]
pub mod polkadot {}

#[derive(Debug, Clone, Parser)]
pub struct OffchainLabOpts {
    #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
    pub substrate_url: Url,

    #[arg(short, long, default_value = "http://127.0.0.1:3030")]
    pub offchain_lab_url: Url,
}

pub async fn run(opts: OffchainLabOpts) -> Result<()> {
    info!("Starting OffchainLab integration");

    let client = OnlineClient::<PolkadotConfig>::from_url(opts.substrate_url.as_str()).await?;
    let offchain_lab_client = reqwest::Client::new();

    let bend_program = fetch_bend_program(&offchain_lab_client, &opts.offchain_lab_url).await?;
    store_bend_program_on_chain(&client, &bend_program).await?;

    info!("OffchainLab integration completed successfully");
    Ok(())
}

async fn fetch_bend_program(client: &reqwest::Client, url: &Url) -> Result<Vec<u8>> {
    info!("Fetching Bend program from OffchainLab");
    let response = client.get(url.join("/bend_program")?).send().await?;
    let program = response.bytes().await?.to_vec();
    info!("Bend program fetched successfully");
    Ok(program)
}

async fn store_bend_program_on_chain(client: &OnlineClient<PolkadotConfig>, program: &[u8]) -> Result<()> {
    info!("Storing Bend program on-chain");
    
    let from = dev::alice();
    let tx = polkadot::tx().bend_program_storage().store_program(program.to_vec());

    let latest_block = client.blocks().at_latest().await?;
    let tx_params = subxt::config::polkadot::PolkadotExtrinsicParamsBuilder::new()
        .tip(1_000)
        .mortal(latest_block.header(), 32)
        .build();

    let tx_hash = client.tx().sign_and_submit(&tx, &from, tx_params).await?;
    info!("Bend program stored on-chain successfully. Transaction hash: {:?}", tx_hash);
    
    Ok(())
}