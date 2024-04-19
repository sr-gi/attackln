use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tonic_lnd::lnrpc;

use attackln::lnd::LndConnection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attackers {
    nodes: Vec<LndConnection>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (mallory, helper) = {
        let attackers: Attackers = serde_json::from_str(
            &std::fs::read_to_string("source.json")
                .map_err(|_| anyhow!("Failed to read config file"))?,
        )
        .map_err(|_| anyhow!("Failed to parse config file"))?;

        (attackers.nodes[0].clone(), attackers.nodes[1].clone())
    };

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut mallory_rpc = tonic_lnd::connect(mallory.address, mallory.cert, mallory.macaroon)
        .await
        .map_err(|_| anyhow!("Failed to connect to rpc server"))?;

    let helper_rpc = tonic_lnd::connect(helper.address, helper.cert, helper.macaroon)
        .await
        .map_err(|_| anyhow!("Failed to connect to rpc server"))?;

    let graph = mallory_rpc
        .lightning()
        .describe_graph(lnrpc::ChannelGraphRequest {
            include_unannounced: true,
        })
        .await
        .map_err(|_| anyhow!("Failed to get graph"))?
        .into_inner();

    // Get all nodes
    let nodes = graph
        .nodes
        .into_iter()
        .map(|node| (node.alias, (node.pub_key, node.addresses[0].addr.clone())))
        .collect::<HashMap<_, _>>();

    let target_node = nodes.get("7").unwrap().clone();

    for (node, mut rpc) in [(mallory.id, mallory_rpc), (helper.id, helper_rpc)]{
        if rpc
            .lightning()
            .connect_peer(lnrpc::ConnectPeerRequest {
                addr: Some(lnrpc::LightningAddress {
                    pubkey: target_node.0.clone(),
                    host: target_node.1.clone(),
                }),
                perm: true,
                timeout: 0,
            })
            .await.is_err() {
                println!("{} connected to peer, skipping", node)
            };
    }



    Ok(())
}
