use crate::{
    info::{
        AssetPosition, Level, MarginSummary, 
        sub_structs::UserState,
        types::{UserFee, SpotMetaAndAssetContexts, UserTokenBalance, Candle, Fill, Trade, FundingRate},
    },
    meta::{SpotMeta, SpotAssetContext, Meta},
    DailyUserVlm, Delta, FeeSchedule, OrderInfo, Referrer, ReferrerState,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStateResponse {
    pub data: UserState,
}

#[derive(Deserialize, Debug)]
pub struct UserTokenBalanceResponse {
    pub data: Vec<UserTokenBalance>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFeesResponse {
    pub data: UserFee,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStatesResponse {
    pub data: Vec<UserState>,
}

#[derive(Deserialize, Debug)]
pub struct UserTokenBalancesResponse {
    pub data: Vec<UserTokenBalance>,
}

#[derive(Deserialize, Debug)]
pub struct FundingHistoryResponse {
    pub data: Vec<FundingRate>,
}

#[derive(Deserialize, Debug)]
pub struct SpotMetaAndAssetContextsResponse {
    pub data: SpotMetaAndAssetContexts,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub data: Vec<OrderInfo>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsResponse {
    pub data: Vec<Fill>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CandlesSnapshotResponse {
    pub data: Vec<Candle>,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatusResponse {
    pub data: OrderInfo,
}

#[derive(Deserialize, Debug)]
pub struct ReferralResponse {
    pub data: ReferrerState,
}

#[derive(Deserialize, Debug)]
pub struct UserFundingResponse {
    pub data: Delta,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct L2SnapshotResponse {
    pub data: L2Data,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct L2Data {
    pub asks: Vec<Level>,
    pub bids: Vec<Level>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradesResponse {
    pub data: Vec<Trade>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MetaResponse {
    pub data: Meta,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SpotMetaResponse {
    pub data: SpotMeta,
}
