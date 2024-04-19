#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address: String = args
        .next()
        .expect("missing arguments: address, macaroon file, payment hash")
        .into_string()
        .expect("address is not UTF-8");
    let cert_file: String = args
        .next()
        .expect("missing arguments: cert file, macaroon file, payment hash")
        .into_string()
        .expect("cert_file is not UTF-8");
    let macaroon_file: String = args
        .next()
        .expect("missing argument: macaroon file, payment hash")
        .into_string()
        .expect("macaroon_file is not UTF-8");
    let payment_hash: Vec<u8> = hex::decode(
        args.next()
            .expect("missing argument: payment hash")
            .into_string()
            .expect("payment_hash is not UTF-8"),
    )
        .expect("payment_hash is not a valid hex");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
    .await
    .expect("failed to connect");

    let invoices = client
        .lightning()
        // All calls require at least empty parameter
        .list_invoices(tonic_lnd::lnrpc::ListInvoiceRequest {pending_only: false, index_offset: 0, num_max_invoices: 0, reversed: false})
        .await
        .expect("failed to get info");

    let mut endorsed = Vec::new();
    for invoice in invoices.into_inner().invoices {
        if invoice.r_hash == payment_hash {
            endorsed =  invoice.htlcs.iter().map(|htlc| (htlc.htlc_index, htlc.incoming_endorsed)).collect::<Vec<_>>();
        }
    }

    if !endorsed.is_empty() {
        println!("{:?}", endorsed);
    } else {
        println!("Cannot find {}", hex::encode(payment_hash));
    }
}