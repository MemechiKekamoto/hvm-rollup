use crate::{Calldata, Result};
use clap::Parser;
use log::{info, error};
use reqwest::Client;
use url::Url;

#[derive(Debug, Clone, Parser)]
pub struct FetchOpts {
    #[arg(short, long)]
    pub sequencer_url: Url,

    #[arg(short = 't', long, default_value = "0", value_name = "NUM")]
    pub threads: usize,

    #[arg(short, long, default_value = "100")]
    pub batch_size: usize,
}

pub async fn fetch(opts: FetchOpts) -> Result<Vec<Calldata>> {
    info!("Fetching calldata from sequencer: {}", opts.sequencer_url);

    let client = Client::new();
    let response = client.get(opts.sequencer_url)
        .query(&[("batch_size", opts.batch_size)])
        .send()
        .await?;

    if !response.status().is_success() {
        error!("Failed to fetch calldata: {}", response.status());
        return Err(crate::error::RelayerError::SequencerError(response.status().to_string()));
    }

    let calldata: Vec<Calldata> = response.json().await?;
    info!("Fetched {} calldata items", calldata.len());

    Ok(calldata)
}