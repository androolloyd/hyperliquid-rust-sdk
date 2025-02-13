use log::info;

use hyperliquid_rust_sdk::{info::InfoClient, helpers::BaseUrl, Message, Subscription};
use tokio::{
    spawn,
    sync::mpsc::unbounded_channel,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut info_client = InfoClient::new_with_ws(BaseUrl::Testnet.get_ws_url()).await.unwrap();

    let (sender, mut receiver) = unbounded_channel();
    let subscription_id = info_client
        .subscribe(
            Subscription::Candle {
                coin: "ETH".to_string(),
                interval: "1m".to_string(),
            },
            sender,
        )
        .await
        .unwrap();

    spawn(async move {
        sleep(Duration::from_secs(300)).await;
        info!("Unsubscribing from candle data");
        info_client.unsubscribe(subscription_id).await.unwrap()
    });

    // This loop ends when we unsubscribe
    while let Some(Message::Candle(candle)) = receiver.recv().await {
        info!("Received candle data: {candle:?}");
    }
}
