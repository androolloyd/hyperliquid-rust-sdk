use alloy_primitives::{Address, U256};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    constants::Chain,
    errors::{HyperliquidError, Result},
    exchange::{
        actions::{
            ApproveAgent, ApproveBuilderFee, BulkCancel, BulkCancelCloid, BulkModify, BulkOrder,
            ClassTransfer, ClaimRewards, CompleteUnstake, SetReferrer, SpotSend, Stake,
            StartUnstake, UpdateIsolatedMargin, UpdateLeverage, UsdSend, VaultTransfer, Withdraw3,
        },
        cancel::{CancelRequest, CancelRequestCloid, ClientCancelRequest, ClientCancelRequestCloid},
        modify::ModifyRequest,
        order::{ClientOrder, Limit, Order, OrderRequest, Trigger, ClientOrderRequest},
        BuilderInfo,
    },
    info::sub_structs::OrderInfo,
    helpers::*,
    signature::{
        agent::Agent,
        create_signature::sign_l1_action,
    },
};

impl From<ClientOrderRequest> for OrderRequest {
    fn from(req: ClientOrderRequest) -> Self {
        OrderRequest {
            asset: req.asset.parse().unwrap_or_default(),
            is_buy: req.is_buy,
            reduce_only: req.reduce_only,
            limit_px: req.limit_px.to_string(),
            sz: req.sz.to_string(),
            cloid: req.cloid.map(|uuid| uuid.to_string()),
            order_type: match req.order_type {
                ClientOrder::Limit(limit) => Order::Limit(Limit {
                    tif: limit.tif,
                }),
                ClientOrder::Trigger(trigger) => Order::Trigger(Trigger {
                    is_market: trigger.is_market,
                    trigger_px: trigger.trigger_px.to_string(),
                    tpsl: trigger.tpsl,
                }),
            },
        }
    }
}

impl From<ClientCancelRequest> for CancelRequest {
    fn from(req: ClientCancelRequest) -> Self {
        CancelRequest {
            asset: req.asset.parse().unwrap_or_default(),
            oid: req.oid,
        }
    }
}

impl From<ClientCancelRequestCloid> for CancelRequestCloid {
    fn from(req: ClientCancelRequestCloid) -> Self {
        CancelRequestCloid {
            asset: req.asset.parse().unwrap_or_default(),
            cloid: req.cloid,
        }
    }
}

pub(crate) enum Actions {
    ApproveBuilderFee(ApproveBuilderFee),
    ApproveAgent(ApproveAgent),
    UsdSend(UsdSend),
    SpotSend(SpotSend),
    ClassTransfer(ClassTransfer),
    SetReferrer(SetReferrer),
    UpdateLeverage(UpdateLeverage),
    UpdateIsolatedMargin(UpdateIsolatedMargin),
    BulkOrder(BulkOrder),
    BulkCancel(BulkCancel),
    BulkModify(BulkModify),
    BulkCancelCloid(BulkCancelCloid),
    Withdraw3(Withdraw3),
    Stake(Stake),
    StartUnstake(StartUnstake),
    CompleteUnstake(CompleteUnstake),
    ClaimRewards(ClaimRewards),
    VaultTransfer(VaultTransfer),

}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ExchangeResponseStatus {
    Ok(ExchangeResponse),
    Err(String),
}

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    pub status: String,
    pub data: Option<ExchangeData>,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeData {
    pub statuses: Vec<ExchangeDataStatus>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ExchangeDataStatus {
    Success,
    Error(String),
    Filled(OrderInfo),
    Resting(OrderInfo),
}

#[derive(Debug)]
pub struct ExchangeClient {
    http_client: Client,
    base_url: String,
}

impl ExchangeClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }

    pub fn get_timestamp(&self) -> U256 {
        U256::from(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs())
    }

    pub async fn usd_send(&self, destination: Address, amount: U256, hyperliquid_chain: String) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = UsdSend {
            signatureChainId: U256::from(421614u64),
            hyperliquidChain: hyperliquid_chain,
            destination,
            amount,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/usdTransfer", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn approve_agent(&self, address: Address, hyperliquid_chain: String) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = ApproveAgent {
            signatureChainId: U256::from(421614u64),
            hyperliquidChain: hyperliquid_chain,
            agent: address,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/approveAgent", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn withdraw(&self, destination: Address, amount: U256, hyperliquid_chain: String) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = Withdraw3 {
            signatureChainId: U256::from(421614u64),
            hyperliquidChain: hyperliquid_chain,
            destination,
            amount,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/withdraw", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn spot_send(&self, destination: Address, token: String, amount: U256, hyperliquid_chain: String) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = SpotSend {
            signatureChainId: U256::from(421614u64),
            hyperliquidChain: hyperliquid_chain,
            destination,
            amount,
            token,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/spotTransfer", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn class_transfer(&self, amount: U256, to_perp: bool, hyperliquid_chain: String) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = ClassTransfer {
            signatureChainId: U256::from(421614u64),
            hyperliquidChain: hyperliquid_chain,
            amount,
            toPerp: to_perp,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/classTransfer", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn cancel(&self, req: ClientCancelRequest, _builder: Option<BuilderInfo>) -> Result<ExchangeResponseStatus> {
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/cancel", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn order(&self, req: ClientOrderRequest, _builder: Option<BuilderInfo>) -> Result<ExchangeResponseStatus> {
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/order", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn set_referrer(&self, code: String) -> Result<()> {
        let req = serde_json::json!({
            "code": code
        });
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/setReferrer", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn cancel_by_cloid(&self, req: ClientCancelRequestCloid, _builder: Option<BuilderInfo>) -> Result<ExchangeResponseStatus> {
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/cancelByCloid", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn vault_transfer(&self, vault_address: Address, is_deposit: bool, usd: String, hyperliquid_chain: String) -> Result<()> {
        let req = VaultTransfer {
            vault_address,
            is_deposit,
            usd,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/vaultTransfer", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn update_leverage(&self, asset: u32, is_cross: bool, leverage: u32) -> Result<()> {
        let req = UpdateLeverage {
            asset,
            is_cross,
            leverage,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/updateLeverage", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn bulk_order(&self, orders: Vec<ClientOrderRequest>, grouping: String, builder: Option<BuilderInfo>) -> Result<ExchangeResponseStatus> {
        let orders: Vec<OrderRequest> = orders.into_iter().map(|o| o.into()).collect();
        let req = BulkOrder {
            orders,
            grouping,
            builder,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/bulkOrder", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn bulk_cancel(&self, cancels: Vec<ClientCancelRequest>) -> Result<ExchangeResponseStatus> {
        let cancels: Vec<CancelRequest> = cancels.into_iter().map(|c| c.into()).collect();
        let req = BulkCancel { cancels };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/bulkCancel", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn bulk_modify(&self, modifies: Vec<ModifyRequest>) -> Result<ExchangeResponseStatus> {
        let req = BulkModify { modifies };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/bulkModify", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn bulk_cancel_cloid(&self, cancels: Vec<ClientCancelRequestCloid>) -> Result<ExchangeResponseStatus> {
        let cancels: Vec<CancelRequestCloid> = cancels.into_iter().map(|c| c.into()).collect();
        let req = BulkCancelCloid { cancels };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        let response = self.http_client
            .post(format!("{}/exchange/bulkCancelCloid", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        let text = response.text().await.map_err(HyperliquidError::from)?;
        serde_json::from_str(&text).map_err(|e| HyperliquidError::SerializationError(e.to_string()))
    }

    pub async fn update_isolated_margin(&self, asset: u32, is_buy: bool, ntli: i64) -> Result<()> {
        let req = UpdateIsolatedMargin {
            asset,
            is_buy,
            ntli,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/updateIsolatedMargin", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn stake(&self, amount: U256, chain: Chain) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = Stake {
            signatureChainId: U256::from(chain.chain_id()),
            hyperliquidChain: chain.as_str().to_string(),
            amount,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/stake", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn start_unstake(&self, amount: U256, chain: Chain) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = StartUnstake {
            signatureChainId: U256::from(chain.chain_id()),
            hyperliquidChain: chain.as_str().to_string(),
            amount,
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/startUnstake", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn complete_unstake(&self, chain: Chain) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = CompleteUnstake {
            signatureChainId: U256::from(chain.chain_id()),
            hyperliquidChain: chain.as_str().to_string(),
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/completeUnstake", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }

    pub async fn claim_rewards(&self, chain: Chain) -> Result<()> {
        let timestamp = self.get_timestamp();
        let req = ClaimRewards {
            signatureChainId: U256::from(chain.chain_id()),
            hyperliquidChain: chain.as_str().to_string(),
            time: timestamp,
        };
        let req_json = serde_json::to_string(&req).map_err(|e| HyperliquidError::SerializationError(e.to_string()))?;
        self.http_client
            .post(format!("{}/exchange/claimRewards", self.base_url))
            .body(req_json)
            .send()
            .await
            .map_err(HyperliquidError::from)?;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use mockito::{Server, ServerGuard};

    fn setup_mock_server() -> (ExchangeClient, ServerGuard) {
        let mut server = Server::new();
        let client = ExchangeClient::new(server.url());
        (client, server)
    }

    #[tokio::test]
    async fn test_stake() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/stake")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\":\"success\"}")
            .create();
        
        let result = client.stake(U256::from(1000000000000000000u64), Chain::Testnet).await;
        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_start_unstake() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/startUnstake")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\":\"success\"}")
            .create();
        
        let result = client.start_unstake(U256::from(1000000000000000000u64), Chain::Testnet).await;
        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_complete_unstake() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/completeUnstake")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\":\"success\"}")
            .create();
        
        let result = client.complete_unstake(Chain::Testnet).await;
        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_claim_rewards() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/claimRewards")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\":\"success\"}")
            .create();
        
        let result = client.claim_rewards(Chain::Testnet).await;
        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_stake_error_response() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/stake")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body("{\"error\":\"Invalid amount\"}")
            .create();
        
        let result = client.stake(U256::from(0), Chain::Testnet).await;
        assert!(result.is_err());
        mock.assert();
    }

    #[tokio::test]
    async fn test_stake_request_validation() {
        let (client, mut server) = setup_mock_server();
        let mock = server.mock("POST", "/exchange/stake")
            .match_body(mockito::Matcher::JsonString(r#"{"signatureChainId":"421614","hyperliquidChain":"arbitrum_testnet","amount":"1000000000000000000","time":"[0-9]+"}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\":\"success\"}")
            .create();
        
        let result = client.stake(U256::from(1000000000000000000u64), Chain::Testnet).await;
        assert!(result.is_ok());
        mock.assert();
    }
}
