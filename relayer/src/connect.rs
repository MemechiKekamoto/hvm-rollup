use crate::Result;
use clap::Parser;
use futures_util::StreamExt;
use log::{info, error};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use url::Url;

#[derive(Debug, Clone, Parser)]
pub struct ConnectOpts {
    #[arg(short, long)]
    pub url: Url,

    #[arg(short, long, default_value = "1")]
    pub threads: usize,
}

pub async fn start(opts: ConnectOpts) -> Result<()> {
    info!("Connecting to {}", opts.url);

    let (ws_stream, _) = connect_async(opts.url).await?;
    info!("WebSocket connection established");

    handle_connection(ws_stream).await
}

async fn handle_connection(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<()> {
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    info!("Received message: {:?}", msg);
                }
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}