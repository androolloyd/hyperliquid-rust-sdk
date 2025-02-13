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

    let usdc = 1000; // 1000 USDC
    let to_perp = true; // Transfer to perp account

    info!(
        "Transferring {} USDC from user {} to {} account",
        usdc,
        user,
        if to_perp { "perp" } else { "spot" }
    );

    let amount = U256::from(usdc);
    
    match exchange_client
        .class_transfer(amount, to_perp, "Testnet".to_string())
        .await {
            Ok(_) => info!("Class transfer completed successfully"),
            Err(e) => error!("Class transfer failed: {}", e),
        }
}
