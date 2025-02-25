#![deny(unreachable_pub)]

// Internal modules
mod constants;
mod errors;
mod exchange;
pub mod helpers;
pub mod info;
mod market_maker;
mod meta;
mod prelude;
mod proxy_digest;
mod req;
mod signature;
mod ws;

// Public re-exports
pub use alloy_primitives::{Address, B256, U256};
pub use alloy_signer_local::PrivateKeySigner as LocalWallet;

pub use constants::{Chain, EPSILON, LOCAL_API_URL, MAINNET_API_URL, TESTNET_API_URL};
pub use errors::Error;
pub use exchange::*;
pub use helpers::{bps_diff, truncate_float, BaseUrl};
pub use info::{
    InfoClient,
    // Other available exports from info
};
pub use market_maker::{MarketMaker, MarketMakerInput, MarketMakerRestingOrder};
pub use meta::{AssetMeta, Meta, SpotMetaAndAssetCtxs, SpotAssetContext};
pub use signature::create_signature::SignatureBytes;
pub use ws::{
    Candle,
    Trade,
    Message,
    Subscription,
    WsManager,
};
