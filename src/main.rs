use std::time::Duration;

use nostr_connect::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app_keys = Keys::generate();
    let relay = RelayUrl::parse("wss://relay.nsec.app")?;

    let uri = NostrConnectURI::client(app_keys.public_key(), [relay], "Amber Debug");
    log::info!("Nostr Connect URI: {uri}");

    let signer = NostrConnect::new(uri, app_keys, Duration::from_secs(300), None)?;

    match signer.bunker_uri().await {
        Ok(uri) => log::info!("Bunker URI: {uri}"),
        Err(e) => log::error!("Error: {e}"),
    }

    Ok(())
}
