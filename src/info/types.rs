use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::info::sub_structs::UserState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenBalance {
    pub token: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFee {
    pub tier: u32,
    pub maker_rate: String,
    pub taker_rate: String,
    pub total_volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FundingRate {
    pub timestamp: u64,
    pub rate: String,
    pub asset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotMetaAsset {
    pub name: String,
    pub decimals: u32,
    pub lot_size: String,
    pub tick_size: String,
    pub min_size: String,
    pub maker_fee: String,
    pub taker_fee: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotMeta {
    pub assets: HashMap<String, SpotMetaAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetContext {
    pub asset: String,
    pub price: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotMetaAndAssetContexts {
    pub meta: SpotMeta,
    pub contexts: Vec<AssetContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fill {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub time: u64,
    pub hash: String,
}

// Response types
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserTokenBalancesResponse {
    pub data: Vec<UserTokenBalance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserFeesResponse {
    pub data: UserFee,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FundingHistoryResponse {
    pub data: Vec<FundingRate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SpotMetaResponse {
    pub data: SpotMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SpotMetaAndAssetContextsResponse {
    pub data: SpotMetaAndAssetContexts,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CandlesResponse {
    pub data: Vec<Candle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserStatesResponse {
    pub data: Vec<UserState>,
} 