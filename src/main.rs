mod vendor;

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

#[derive(Serialize, Deserialize)]
pub struct Config {
    rpc_url: String,
    keypair_file: Option<String>,
    bind: SocketAddr,
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config: Config = Figment::new().merge(Env::raw()).extract().unwrap();

    let rpc = RpcClient::new(config.rpc_url);
    let slot = rpc.get_slot().await.unwrap();
    let leader_schedule = rpc.get_leader_schedule(Some(slot)).await.unwrap().unwrap();

    let leader_pks = leader_schedule.keys().collect::<HashSet<_>>();
    let nodes = rpc.get_cluster_nodes().await.unwrap();

    let leader_gossip_map = nodes
        .iter()
        .filter(|node| leader_pks.contains(&node.pubkey))
        .map(|node| (node.pubkey.clone(), node.clone()))
        .collect::<HashMap<_, _>>();

    //try to connect to the tpu port of one of them
    let doggo_pk = pubkey!("SscQkTYV2BFQYGGffAmTzvefrFrw6z9GNYiWHstVZ77");
    let doggo_node_info = leader_gossip_map.get(&doggo_pk.to_string()).unwrap();

    println!("{:?}", doggo_node_info);

    let keypair = if let Some(keypair_file) = config.keypair_file {
        Keypair::read_from_file(keypair_file).unwrap()
    } else {
        Keypair::new()
    };

    let client_certificate = Arc::new(QuicClientCertificate::new(&keypair));
    let client_config = create_client_config(client_certificate);
    let endpoint = create_client_endpoint(config.bind, client_config).unwrap();
    let connect = endpoint.connect(doggo_node_info.tpu.unwrap(), "connect");
    match connect {
        Ok(connecting) => {
            let connection_res = connecting.await;
            match connection_res {
                Ok(connection) => {
                    println!("Connected to doggo");
                }
                Err(e) => {
                    println!("Failed to connect to doggo: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to doggo: {:?}", e);
        }
    }
}
