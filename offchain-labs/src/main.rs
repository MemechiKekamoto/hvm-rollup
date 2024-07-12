use offchain_labs::{Config, OffchainLabs};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}. Using default configuration.", e);
        Config::default()
    });
    
    let mut hvm = OffchainLabs::new(config)?;

    info!("OffchainLabs initialized");

    let transactions = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];

    for (i, tx) in transactions.iter().enumerate() {
        match hvm.process_transaction(tx) {
            Ok(is_valid) => {
                info!("Transaction {} processed. Valid: {}", i, is_valid);
            }
            Err(e) => {
                error!("Error processing transaction {}: {}", i, e);
            }
        }
    }

    info!("All transactions processed");

    Ok(())
}