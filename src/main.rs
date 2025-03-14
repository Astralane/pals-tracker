#![feature(duration_constructors)]

mod app;
mod constants;
mod palidator_cache;
mod palidator_tracker;
mod server;
mod vendor;

use crate::palidator_tracker::PalidatorTracker;
use crate::vendor::quic_client_certificate::QuicClientCertificate;
use crate::vendor::quic_networking::{create_client_config, create_client_endpoint};
use figment::Figment;
use figment::providers::Env;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::signature::{EncodableKey, Keypair};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    rpc_url: String,
    ws_url: String,
    keypair_file: Option<String>,
    quic_bind: SocketAddr,
    address: SocketAddr,
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let config: Config = Figment::new().merge(Env::raw()).extract().unwrap();
    info!("config: {:?}", config);

    let rpc = Arc::new(RpcClient::new(config.rpc_url));
    let cancel = CancellationToken::new();

    let keypair = if let Some(keypair_file) = config.keypair_file {
        Keypair::read_from_file(keypair_file).unwrap()
    } else {
        Keypair::new()
    };

    let tracker = PalidatorTracker::new(
        rpc.clone(),
        config.ws_url,
        keypair,
        config.quic_bind,
        cancel.clone(),
    )
    .await
    .unwrap();

    let hdl = server::serve(
        config.address,
        tracker.palidator_cache.clone(),
        tracker.slot.clone(),
    )
    .await
    .unwrap();

    tracker.join().await;
}
