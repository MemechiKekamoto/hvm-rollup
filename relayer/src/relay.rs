use crate::{Calldata, RelayerError, Result, Stats};
use clap::Parser;
use log::{info, error};
use sp_core::H256;
use subxt::{
    OnlineClient,
    PolkadotConfig,
    Config,
};
use subxt::config::polkadot::PolkadotExtrinsicParamsBuilder as Params;
use subxt::config::ExtrinsicParams;
use subxt_signer::sr25519::dev;
use url::Url;

// #[subxt::subxt(runtime_metadata_path = "./artifacts/polkadot_metadata_small.scale")]
// pub mod polkadot {}

#[derive(Debug, Clone, Parser)]
pub struct RelayOpts {
    #[arg(short, long)]
    pub substrate_url: Url,

    #[arg(short, long)]
    pub sequencer_url: Url,

    #[arg(short = 't', long, default_value = "0", value_name = "NUM")]
    pub threads: usize,

    #[arg(short, long, default_value = "100")]
    pub batch_size: usize,
}

pub async fn start(opts: RelayOpts) -> Result<()> {
    info!("Starting relay process");

    let client = OnlineClient::<PolkadotConfig>::from_url(opts.substrate_url.as_str()).await?;

    let calldata = crate::calldata::fetch(crate::calldata::FetchOpts {
        sequencer_url: opts.sequencer_url,
        threads: opts.threads,
        batch_size: opts.batch_size,
    }).await?;

    let stats = relay_calldata(&client, calldata).await?;

    info!("Relay process completed. Stats: {:?}", stats);

    Ok(())
}

async fn relay_calldata<T: Config>(client: &OnlineClient<T>, calldata: Vec<Calldata>) -> Result<Stats>
where
    T: Config,
    T::ExtrinsicParams: ExtrinsicParams<T>,
    <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
{
    let mut stats = Stats { total: calldata.len(), success: 0, error: 0 };

    for data in calldata {
        match send_extrinsic(client, &data).await {
            Ok(_) => stats.success += 1,
            Err(e) => {
                error!("Failed to send extrinsic: {}", e);
                stats.error += 1;
            }
        }
    }

    Ok(stats)
}

async fn send_extrinsic<T: Config>(client: &OnlineClient<T>, calldata: &Calldata) -> Result<H256>
where
    T: Config,
    T::ExtrinsicParams: ExtrinsicParams<T>,
    <<T as Config>::ExtrinsicParams as ExtrinsicParams<T>>::OtherParams: Default,
{
    Ok(H256::zero())
}
