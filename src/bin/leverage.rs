use alloy_primitives::U256;
use alloy_signer_local::PrivateKeySigner;
use hyperliquid_rust_sdk::{BaseUrl, ExchangeClient};
use log::{error, info};

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let priv_key = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e";
    let wallet = priv_key.parse::<PrivateKeySigner>().unwrap();
    let user = wallet.address();

    let exchange_client = ExchangeClient::new(BaseUrl::Testnet.get_url());

    // Example: Set 10x leverage for ETH in cross margin mode
    let asset = 0; // ETH asset ID
    let is_cross = true;
    let leverage = 10;

    info!(
        "Setting {}x leverage for asset {} in {} mode for user {}",
        leverage,
        asset,
        if is_cross { "cross" } else { "isolated" },
        user
    );

    match exchange_client.update_leverage(asset, is_cross, leverage).await {
        Ok(_) => info!("Successfully updated leverage"),
        Err(e) => error!("Failed to update leverage: {}", e),
    }
}
