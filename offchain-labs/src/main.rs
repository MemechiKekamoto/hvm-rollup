use offchain_labs::{config::SequencerConfig, sequencer::{ Sequencer, Transaction}, zk_rollup::Proof, Config, OffchainLabs};
use log::{info, error};
use axum::{
    routing::{ get, post },
    http::StatusCode,
    Json, Router, Form,
    extract::{ Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;
use offchain_labs::zk_rollup::{State as ZkState};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init();

    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}. Using default configuration.", e);
        Config::default()
    });
    
    let mut hvm = OffchainLabs::new(config)?;

    info!("OffchainLabs initialized");
    let app = Router::<OffchainLabs>::new()
    .route("/submit_tx", post(submit_transaction))
    .route("/sequencer", post(submit_to_sequencer))
    .route("/get_keys", get(get_zk_keys))
    .with_state(hvm);

    let mut PORT = env::var(if cfg!(debug_assertions) {"PORT_DEV"} else {"PORT_PROD"}).unwrap();

    let log = format!("Listening on port {}", PORT);
    info!("{}", log);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await.unwrap();
    axum::serve(listener, app).await.unwrap();


    Ok(())
}

async fn get_zk_keys(
    State(state): State<OffchainLabs>
) -> impl IntoResponse {
    
    let pk = format!("{:?}", state.prover.proving_key);
    let vk = format!("{:?}", state.verifier.verifying_key);
    Json(ZkKeys {
        pk,
        vk
    })
}

#[derive(Serialize)]
struct ZkKeys {
    pk: String,
    vk: String
}

async fn submit_to_sequencer(
    State(state): State<OffchainLabs>,
    Form(payload): Form<SubmitTransaction>,
) -> impl IntoResponse {
    let tx_data: TransactionData = serde_json::from_str(&payload.raw_transaction).unwrap();
    let transaction = Transaction::new(tx_data.from, tx_data.to, 0, 1);
    let mut seq = Sequencer::new(ZkState {
        balance: 0,
        nonce: 0
    }, SequencerConfig {
        max_pending_transactions: 1000,
        max_pending_programs: 1000,
        batch_interval_seconds: 500,
        max_batch_size: 1000,
        max_programs_per_batch: 1000
    });
    seq.process_transaction(transaction).unwrap();
    seq.create_batch(true).unwrap();
    Json(())
}

async fn submit_transaction(
    State(mut state): State<OffchainLabs>,
    Form(payload): Form<SubmitTransaction>,
) -> Result<Json<Option<Proof>>, (StatusCode, String)> {
    
    let tx_data: TransactionData = serde_json::from_str(&payload.raw_transaction).unwrap();
    
    let transaction = Transaction::new(tx_data.from, tx_data.to, 0, 1);
    match state.process_transaction_ex(transaction.clone()) {
        Ok((_is_valid, proof)) => {
            Ok(Json(proof))
        }
        Err(e) => {
            error!("Error processing transaction {}", e);
            Err(internal_error(e))
        }
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SubmitTransaction {
    raw_transaction: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TransactionData {
    txhash: String,
    timestamp: u64,
    r#type: String,
    from: String,
    to: String,
    details: String
}