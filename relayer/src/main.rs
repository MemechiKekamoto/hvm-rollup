use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use web3::transports::Http;
use web3::Web3;
use log::{info, error};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
struct Calldata {
    data: String,
    proof: String,

struct Relayer {
    cache: Arc<Mutex<VecDeque<(String, Calldata)>>>,
    web3: Web3<Http>,
    client: Client,
    sequencer_url: String,
}

impl Relayer {
    fn new(web3_url: &str, sequencer_url: &str) -> Result<Self, Box<dyn Error>> {
        let transport = Http::new(web3_url)?;
        let web3 = Web3::new(transport);
        let client = Client::new();
        Ok(Relayer {
            cache: Arc::new(Mutex::new(VecDeque::new())),
            web3,
            client,
            sequencer_url: sequencer_url.to_string(),
        })
    }

    async fn verify_calldata(&self, calldata: &Calldata) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    async fn generate_tx_hash(&self, calldata: &Calldata) -> Result<String, Box<dyn Error>> {
        let hash = format!("tx_hash_{}", &calldata.data);
        Ok(hash)
    }

    async fn send_tx_hash_to_blockchain(&self, tx_hash: &str) -> Result<(), Box<dyn Error>> {
        println!("Sending tx_hash: {}", tx_hash);
        Ok(())
    }

    async fn process_calldata(&self, calldata: Calldata) -> Result<(), Box<dyn Error>> {
        if !self.verify_calldata(&calldata).await? {
            return Err("Calldata verification failed".into());
        }

        let tx_hash = self.generate_tx_hash(&calldata).await?;

        let mut cache = self.cache.lock().unwrap();
        cache.push_back((tx_hash.clone(), calldata));

        self.send_tx_hash_to_blockchain(&tx_hash).await
    }

    async fn fetch_calldata_from_sequencer(&self) -> Result<Calldata, Box<dyn Error>> {
        let response = self.client.get(&self.sequencer_url).send().await?;
        let calldata: Calldata = response.json().await?;
        Ok(calldata)
    }

    fn handle_massive_overload(&self) {
        let mut cache = self.cache.lock().unwrap();
        if cache.len() > 10000 {
            cache.pop_front();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let relayer = Relayer::new("http://localhost:8545", "http://localhost:8080/sequencer")?;

    match relayer.fetch_calldata_from_sequencer().await {
        Ok(calldata) => {
            if let Err(e) = relayer.process_calldata(calldata).await {
                error!("Error processing calldata: {:?}", e);
            }
        }
        Err(e) => {
            error!("Error fetching calldata: {:?}", e);
        }
    }

    relayer.handle_massive_overload();

    Ok(())
}
