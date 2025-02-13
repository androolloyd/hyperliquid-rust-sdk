pub const MAINNET_API_URL: &str = "https://api.hyperliquid.xyz";
pub const TESTNET_API_URL: &str = "https://api.testnet.hyperliquid.xyz";
pub const LOCAL_API_URL: &str = "http://localhost:3001";

pub(crate) const ARBITRUM_CHAIN_ID: u64 = 42161;
pub(crate) const ARBITRUM_TESTNET_CHAIN_ID: u64 = 421614;

pub const EPSILON: f64 = 1e-9;
pub(crate) const INF_BPS: u16 = 10_001;

// Chain identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chain {
    Mainnet,
    Testnet,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::Mainnet => ARBITRUM_CHAIN_ID,
            Chain::Testnet => ARBITRUM_TESTNET_CHAIN_ID,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Chain::Mainnet => "arbitrum",
            Chain::Testnet => "arbitrum_testnet",
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain::Mainnet
    }
}

// Websocket URLs
pub(crate) const MAINNET_WS_URL: &str = "wss://api.hyperliquid.xyz/ws";
pub(crate) const TESTNET_WS_URL: &str = "wss://api.testnet.hyperliquid.xyz/ws";

// Time constants
pub(crate) const MINUTE: u64 = 60;
pub(crate) const HOUR: u64 = MINUTE * 60;
pub(crate) const DAY: u64 = HOUR * 24;

// API rate limits
pub(crate) const RATE_LIMIT_PER_MINUTE: u32 = 600;
pub(crate) const WS_RATE_LIMIT_PER_MINUTE: u32 = 100;

// Order related constants
pub(crate) const MAX_LEVERAGE: u32 = 50;
pub(crate) const MIN_ORDER_SIZE: f64 = 0.0001;
pub(crate) const MAX_ORDERS_PER_REQUEST: usize = 100;

// Decimal precision
pub(crate) const USD_DECIMALS: u32 = 6;
pub(crate) const PRICE_DECIMALS: u32 = 8;
pub(crate) const SIZE_DECIMALS: u32 = 8; 