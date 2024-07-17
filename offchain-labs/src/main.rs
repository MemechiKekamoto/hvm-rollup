use offchain_labs::{Config, OffchainLabs, sequencer::Transaction};
use log::{info, error};

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
        Transaction::new("Alice".to_string(), "Bob".to_string(), 100, 1),
        Transaction::new("Bob".to_string(), "Charlie".to_string(), 50, 1),
        Transaction::new("Charlie".to_string(), "Alice".to_string(), 25, 1),
    ];

    for (i, tx) in transactions.iter().enumerate() {
        match hvm.process_transaction(tx.clone()) {
            Ok(is_valid) => {
                info!("Transaction {} processed. Valid: {}", i, is_valid);
            }
            Err(e) => {
                error!("Error processing transaction {}: {}", i, e);
            }
        }
    }

    info!("All transactions processed");
    info!("Pending transactions: {}", hvm.pending_transactions_count());
    info!("Current state: {:?}", hvm.get_current_state()?);

    Ok(())
}