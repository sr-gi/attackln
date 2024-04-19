use crate::lnd::LndConnection;
use tonic_lnd::ConnectError;

#[tokio::main]
async fn open_channel(
    node_ath: LndConnection,
    destination: String,
    amt: u64,
) -> Result<(), ConnectError> {
    let mut client = tonic_lnd::connect(node_ath.address, node_ath.cert, node_ath.macaroon).await?;

    Ok(())
}
