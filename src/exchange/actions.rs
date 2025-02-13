use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

use super::{
    cancel::{CancelRequest, CancelRequestCloid}, 
    modify::ModifyRequest, 
    order::OrderRequest,
    BuilderInfo
};

pub(crate) const HYPERLIQUID_EIP_PREFIX: &str = "HyperliquidTransaction:";

pub(crate) mod types {
    use super::*;
    
    sol! {
        #[derive(Debug, Serialize, Deserialize)]
        struct UsdSend {
            uint256 signatureChainId;
            string hyperliquidChain;
            address destination;
            uint256 amount;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ApproveAgent {
            uint256 signatureChainId;
            string hyperliquidChain;
            address agent;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Withdraw3 {
            uint256 signatureChainId;
            string hyperliquidChain;
            address destination;
            uint256 amount;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct SpotSend {
            uint256 signatureChainId;
            string hyperliquidChain;
            address destination;
            string token;
            uint256 amount;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ClassTransfer {
            uint256 signatureChainId;
            string hyperliquidChain;
            uint256 amount;
            bool toPerp;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Stake {
            uint256 signatureChainId;
            string hyperliquidChain;
            uint256 amount;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct StartUnstake {
            uint256 signatureChainId;
            string hyperliquidChain;
            uint256 amount;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct CompleteUnstake {
            uint256 signatureChainId;
            string hyperliquidChain;
            uint256 time;
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ClaimRewards {
            uint256 signatureChainId;
            string hyperliquidChain;
            uint256 time;
        }
    }
}

pub(crate) use types::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: Address,
    pub is_deposit: bool,
    pub usd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
    pub signature_chain_id: U256,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Actions {
    UpdateLeverage(UpdateLeverage),
    UpdateIsolatedMargin(UpdateIsolatedMargin),
    BulkOrder(BulkOrder),
    BulkCancel(BulkCancel),
    BulkModify(BulkModify),
    BulkCancelCloid(BulkCancelCloid),
    VaultTransfer(VaultTransfer),
    SetReferrer(SetReferrer),
    ApproveBuilderFee(ApproveBuilderFee),
    Stake(Stake),
    StartUnstake(StartUnstake),
    CompleteUnstake(CompleteUnstake),
    ClaimRewards(ClaimRewards),
}
