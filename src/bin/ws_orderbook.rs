use hyperliquid_rust_sdk::{info::InfoClient, helpers::BaseUrl};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut info_client = InfoClient::new_with_ws(BaseUrl::Testnet.get_ws_url()).await.unwrap();
// ... existing code ...
} 