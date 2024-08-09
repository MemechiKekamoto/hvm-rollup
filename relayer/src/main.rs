use clap::Parser;
use hvm_relayer::{commands, calldata, connect, relay, runtime, offchain_lab};
use log::info;

#[derive(Debug, Parser)]
#[command(name = "hvm_relayer", about = "HVM offchain relayer", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Connect(connect::ConnectOpts),
    #[command(arg_required_else_help = true)]
    FetchCalldata(calldata::FetchOpts),
    #[command(arg_required_else_help = true)]
    Relay(relay::RelayOpts),
    #[command(arg_required_else_help = true)]
    OffchainLab(offchain_lab::OffchainLabOpts),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Connect(opts) => connect::start(opts).await?,
        Commands::FetchCalldata(opts) => {
            let calldata = calldata::fetch(opts).await?;
            info!("Fetched {} calldata items", calldata.len());
        }
        Commands::Relay(opts) => relay::run(opts).await?,
        Commands::OffchainLab(opts) => offchain_lab::run(opts).await?,
    }

    Ok(())
}