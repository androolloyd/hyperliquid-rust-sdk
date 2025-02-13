pub const MAINNET_API_URL: &str = "https://api.hyperliquid.xyz";
pub const TESTNET_API_URL: &str = "https://api.testnet.hyperliquid.xyz";

pub const ARBITRUM_CHAIN_ID: u64 = 42161;
pub const ARBITRUM_TESTNET_CHAIN_ID: u64 = 421614;

// Websocket URLs
pub const MAINNET_WS_URL: &str = "wss://api.hyperliquid.xyz/ws";
pub const TESTNET_WS_URL: &str = "wss://api.testnet.hyperliquid.xyz/ws";

// Time constants
pub const MINUTE: u64 = 60;
pub const HOUR: u64 = MINUTE * 60;
pub const DAY: u64 = HOUR * 24;

// API rate limits
pub const RATE_LIMIT_PER_MINUTE: u32 = 600;
pub const WS_RATE_LIMIT_PER_MINUTE: u32 = 100;

// Order related constants
pub const MAX_LEVERAGE: u32 = 50;
pub const MIN_ORDER_SIZE: f64 = 0.0001;
pub const MAX_ORDERS_PER_REQUEST: usize = 100;

// Decimal precision
pub const USD_DECIMALS: u32 = 6;
pub const PRICE_DECIMALS: u32 = 8;
pub const SIZE_DECIMALS: u32 = 8; 