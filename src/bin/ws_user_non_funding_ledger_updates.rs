use alloy_primitives::Address;
use hyperliquid_rust_sdk::{info::InfoClient, helpers::BaseUrl, Message, Subscription};
use log::info;
use tokio::{
    spawn,
    sync::mpsc::unbounded_channel,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut info_client = InfoClient::new_with_ws(BaseUrl::Testnet.get_url()).await.unwrap();
    let user = "0xc64cc00b46101bd40aa1c3121195e85c0b0918d8".parse::<Address>().unwrap();

    let (sender, mut receiver) = unbounded_channel();
    let subscription_id = info_client
        .subscribe(Subscription::UserNonFundingLedgerUpdates { user }, sender)
        .await
        .unwrap();

    spawn(async move {
        sleep(Duration::from_secs(30)).await;
        info!("Unsubscribing from user non funding ledger update data");
        info_client.unsubscribe(subscription_id).await.unwrap()
    });

    // this loop ends when we unsubscribe
    while let Some(Message::UserNonFundingLedgerUpdates(user_non_funding_ledger_update)) =
        receiver.recv().await
    {
        info!("Received user non funding ledger update data: {user_non_funding_ledger_update:?}");
    }
}
