use bitcoin::secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Serialize, Debug, Clone)]
pub enum NodeId {
    PublicKey(PublicKey),
    Alias(String),
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NodeId::PublicKey(pk) => pk.to_string(),
                NodeId::Alias(a) => a.to_owned(),
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LndConnection {
    #[serde(with = "crate::serializers::serde_node_id")]
    pub id: NodeId,
    pub address: String,
    #[serde(deserialize_with = "crate::serializers::deserialize_path")]
    pub macaroon: String,
    #[serde(deserialize_with = "crate::serializers::deserialize_path")]
    pub cert: String,
}
