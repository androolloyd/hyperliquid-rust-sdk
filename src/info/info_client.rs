use crate::{
    info::{
        response_structs::{
            CandlesSnapshotResponse, L2SnapshotResponse, OpenOrdersResponse,
            OrderStatusResponse, RecentTradesResponse, UserFillsResponse, UserStateResponse,
            UserFeesResponse, UserStatesResponse, UserTokenBalancesResponse, SpotMetaAndAssetContextsResponse,
            ReferralResponse, MetaResponse, SpotMetaResponse,
        },
        types::{UserFee, UserTokenBalance, FundingRate, SpotMetaAndAssetContexts, SpotMetaAsset, AssetContext, Candle, Fill, Trade},
        sub_structs::{UserState, MarginSummary, OrderInfo},
    },
    meta::{Meta, SpotMeta},
    prelude::*,
    req::HttpClient,
    ws::{Subscription, WsManager},
    BaseUrl, Message,
    errors::{HyperliquidError, Result},
};

use alloy_primitives::Address;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CandleSnapshotRequest {
    pub coin: String,
    pub interval: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub(crate) enum InfoRequest {
    #[serde(rename = "clearinghouseState")]
    UserState {
        user: Address,
    },
    #[serde(rename = "batchClearinghouseStates")]
    UserStates {
        users: Vec<Address>,
    },
    #[serde(rename = "spotClearinghouseState")]
    UserTokenBalances {
        user: Address,
    },
    UserFees {
        user: Address,
    },
    OpenOrders {
        user: Address,
    },
    OrderStatus {
        user: Address,
        oid: u64,
    },
    Meta,
    SpotMeta,
    SpotMetaAndAssetCtxs,
    AllMids,
    UserFills {
        user: Address,
    },
    #[serde(rename_all = "camelCase")]
    FundingHistory {
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    UserFunding {
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    },
    L2Book {
        coin: String,
    },
    RecentTrades {
        coin: String,
    },
    #[serde(rename_all = "camelCase")]
    CandleSnapshot {
        req: CandleSnapshotRequest,
    },
    Referral {
        user: Address,
    },
    HistoricalOrders {
        user: Address,
    },
}

#[derive(Debug)]
pub struct InfoClient {
    pub http_client: HttpClient,
    pub(crate) ws_manager: Option<WsManager>,
    reconnect: bool,
}

impl InfoClient {
    pub async fn new(client: Option<Client>, base_url: Option<BaseUrl>) -> Result<InfoClient> {
        Self::new_internal(client, base_url, false).await
    }

    pub async fn with_reconnect(
        client: Option<Client>,
        base_url: Option<BaseUrl>,
    ) -> Result<InfoClient> {
        Self::new_internal(client, base_url, true).await
    }

    async fn new_internal(
        client: Option<Client>,
        base_url: Option<BaseUrl>,
        reconnect: bool,
    ) -> Result<InfoClient> {
        let client = client.unwrap_or_default();
        let base_url = base_url.unwrap_or(BaseUrl::Mainnet).get_url();

        Ok(InfoClient {
            http_client: HttpClient { client, base_url },
            ws_manager: None,
            reconnect,
        })
    }

    pub async fn subscribe(
        &mut self,
        subscription: Subscription,
        sender_channel: UnboundedSender<Message>,
    ) -> Result<u32> {
        if self.ws_manager.is_none() {
            let ws_manager = WsManager::new(
                format!("ws{}/ws", &self.http_client.base_url[4..]),
                self.reconnect,
            )
            .await
            .map_err(HyperliquidError::from)?;
            self.ws_manager = Some(ws_manager);
        }

        let identifier =
            serde_json::to_string(&subscription).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;

        self.ws_manager
            .as_mut()
            .ok_or_else(|| HyperliquidError::WebsocketError("WS manager not found".to_string()))?
            .add_subscription(identifier, sender_channel)
            .await
            .map_err(HyperliquidError::from)
    }

    pub async fn unsubscribe(&mut self, subscription_id: u32) -> Result<()> {
        if self.ws_manager.is_none() {
            let ws_manager = WsManager::new(
                format!("ws{}/ws", &self.http_client.base_url[4..]),
                self.reconnect,
            )
            .await
            .map_err(HyperliquidError::from)?;
            self.ws_manager = Some(ws_manager);
        }

        self.ws_manager
            .as_mut()
            .ok_or_else(|| HyperliquidError::WebsocketError("WS manager not found".to_string()))?
            .remove_subscription(subscription_id)
            .await
            .map_err(HyperliquidError::from)
    }

    async fn send_info_request<T: for<'a> Deserialize<'a>>(
        &self,
        info_request: InfoRequest,
    ) -> Result<T> {
        let data =
            serde_json::to_string(&info_request).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;

        let return_data = self.http_client.post("/info", data).await?;
        serde_json::from_str(&return_data).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn open_orders(&self, address: Address) -> Result<Vec<OrderInfo>> {
        let input = InfoRequest::OpenOrders { user: address };
        let response: OpenOrdersResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn user_state(&self, address: Address) -> Result<UserStateResponse> {
        let input = InfoRequest::UserState { user: address };
        self.send_info_request(input).await
    }

    pub async fn user_states(&self, addresses: Vec<Address>) -> Result<Vec<UserState>> {
        let response: UserStatesResponse = self.send_info_request(InfoRequest::UserStates { users: addresses.clone() }).await?;
        Ok(response.data)
    }

    pub async fn user_token_balances(&self, address: Address) -> Result<Vec<UserTokenBalance>> {
        let response: UserTokenBalancesResponse = self.send_info_request(InfoRequest::UserTokenBalances { user: address }).await?;
        Ok(response.data)
    }

    pub async fn user_fees(&self, address: Address) -> Result<UserFee> {
        let response: UserFeesResponse = self.send_info_request(InfoRequest::UserFees { user: address }).await?;
        Ok(response.data)
    }

    pub async fn meta(&self) -> Result<Meta> {
        let input = InfoRequest::Meta;
        let response: MetaResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn spot_meta(&self) -> Result<SpotMeta> {
        let input = InfoRequest::SpotMeta;
        let response: SpotMetaResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn spot_meta_and_asset_contexts(&self) -> Result<SpotMetaAndAssetContexts> {
        let response: SpotMetaAndAssetContextsResponse = self.send_info_request(InfoRequest::SpotMetaAndAssetCtxs).await?;
        Ok(response.data)
    }

    pub async fn all_mids(&self) -> Result<HashMap<String, String>> {
        let input = InfoRequest::AllMids;
        self.send_info_request(input).await
    }

    pub async fn user_fills(&self, address: Address) -> Result<Vec<Fill>> {
        let input = InfoRequest::UserFills { user: address };
        let response: UserFillsResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn funding_history(
        &self,
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingRate>> {
        let input = InfoRequest::FundingHistory {
            coin,
            start_time,
            end_time,
        };
        self.send_info_request(input).await
    }

    pub async fn user_funding_history(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingRate>> {
        let input = InfoRequest::UserFunding {
            user,
            start_time,
            end_time,
        };
        self.send_info_request(input).await
    }

    pub async fn recent_trades(&self, coin: String) -> Result<Vec<Trade>> {
        let input = InfoRequest::RecentTrades { coin };
        let response: RecentTradesResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn l2_snapshot(&self, coin: String) -> Result<L2SnapshotResponse> {
        let input = InfoRequest::L2Book { coin };
        self.send_info_request(input).await
    }

    async fn get_candles_snapshot(
        &self,
        coin: String,
        interval: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<CandlesSnapshotResponse> {
        let input = InfoRequest::CandleSnapshot {
            req: CandleSnapshotRequest {
                coin,
                interval,
                start_time,
                end_time,
            },
        };
        self.send_info_request(input).await
    }

    pub async fn candles_snapshot(
        &self,
        coin: String,
        interval: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<Candle>> {
        let response: CandlesSnapshotResponse = self.get_candles_snapshot(coin, interval, start_time, end_time).await?;
        Ok(response.data)
    }

    pub async fn query_order_by_oid(&self, user: Address, oid: u64) -> Result<OrderInfo> {
        let input = InfoRequest::OrderStatus {
            user,
            oid,
        };
        let response: OrderStatusResponse = self.send_info_request(input).await?;
        Ok(response.data)
    }

    pub async fn query_referral_state(&self, address: Address) -> Result<ReferralResponse> {
        let input = InfoRequest::Referral { user: address };
        self.send_info_request(input).await
    }

    pub async fn historical_orders(&self, address: Address) -> Result<Vec<OrderInfo>> {
        let input = InfoRequest::HistoricalOrders { user: address };
        self.send_info_request(input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Address;
    use std::str::FromStr;

    const TEST_ADDRESS: &str = "0x0D1d9635D0640821d15e323ac8AdADfA9c111414";

    async fn get_test_client() -> InfoClient {
        InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap()
    }

    #[tokio::test]
    async fn test_user_state() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_state(address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_states() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_states(vec![address]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_token_balances() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_token_balances(address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_fees() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_fees(address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_meta() {
        let client = get_test_client().await;
        let result = client.meta().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_spot_meta() {
        let client = get_test_client().await;
        let result = client.spot_meta().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_spot_meta_and_asset_contexts() {
        let client = get_test_client().await;
        let result = client.spot_meta_and_asset_contexts().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_all_mids() {
        let client = get_test_client().await;
        let result = client.all_mids().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_fills() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_fills(address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_funding_history() {
        let client = get_test_client().await;
        let result = client.funding_history("ETH".to_string(), 1690393044548, Some(1690393044548 + 3600)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_user_funding_history() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.user_funding_history(address, 1690393044548, Some(1690393044548 + 3600)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_recent_trades() {
        let client = get_test_client().await;
        let result = client.recent_trades("ETH".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_l2_snapshot() {
        let client = get_test_client().await;
        let result = client.l2_snapshot("ETH".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_candles_snapshot() {
        let client = get_test_client().await;
        let result = client.candles_snapshot("ETH".to_string(), "1m".to_string(), 1690393044548, 1690393044548 + 3600).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_order_by_oid() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.query_order_by_oid(address, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_referral_state() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.query_referral_state(address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_historical_orders() {
        let client = get_test_client().await;
        let address = Address::from_str(TEST_ADDRESS).unwrap();
        let result = client.historical_orders(address).await;
        assert!(result.is_ok());
    }
}
