use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Client request error: {message} (code: {error_code:?}, data: {error_data:?})")]
    ClientRequest {
        message: String,
        error_code: Option<i64>,
        error_data: Option<String>,
    },

    #[error("Server request error: {message}")]
    ServerRequest {
        message: String,
    },

    #[error("Chain not allowed")]
    ChainNotAllowed,

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Alloy conversion error: {0}")]
    AlloyConversion(String),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("EIP712 error: {0}")]
    Eip712(String),

    #[error("Generic parse error: {0}")]
    GenericParse(String),

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("Vault address not found: {0}")]
    VaultAddressNotFound(String),

    #[error("Float string parse error: {0}")]
    FloatStringParse(String),

    #[error("Generic request error: {0}")]
    GenericRequest(String),

    #[error("Subscription not found")]
    SubscriptionNotFound,

    #[error("WS manager not instantiated")]
    WsManagerNotFound,

    #[error("WS send error: {0:?}")]
    WsSend(String),

    #[error("Reader data not found")]
    ReaderDataNotFound,

    #[error("Reader error: {0:?}")]
    GenericReader(String),

    #[error("Reader text conversion error: {0:?}")]
    ReaderTextConversion(String),

    #[error("Order type not found")]
    OrderTypeNotFound,

    #[error("Issue with generating random data: {0:?}")]
    RandGen(String),

    #[error("Private key parse error: {0}")]
    PrivateKeyParse(String),

    #[error("Cannot subscribe to multiple user events")]
    UserEvents,

    #[error("RMP parse error: {0}")]
    RmpParse(String),

    #[error("JSON parse error: {0}")]
    JsonParse(String),

    #[error("Websocket error: {0}")]
    Websocket(String),

    #[error("Signature failure: {0}")]
    SignatureFailure(String),

    #[error("Alloy signer error: {0}")]
    AlloySignerError(String),
}

impl From<alloy_signer::Error> for Error {
    fn from(err: alloy_signer::Error) -> Self {
        Error::AlloySignerError(err.to_string())
    }
}

impl From<fn(String) -> Error> for Error {
    fn from(_: fn(String) -> Error) -> Self {
        Error::AssetNotFound("Asset not found".to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HyperliquidError {
    #[error("Invalid asset: {0}")]
    InvalidAsset(String),

    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    #[error("Invalid size: {0}")]
    InvalidSize(String),

    #[error("Invalid leverage: {0}")]
    InvalidLeverage(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Websocket error: {0}")]
    WebsocketError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Order error: {0}")]
    OrderError(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Position error: {0}")]
    PositionError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for HyperliquidError {
    fn from(err: reqwest::Error) -> Self {
        HyperliquidError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for HyperliquidError {
    fn from(err: serde_json::Error) -> Self {
        HyperliquidError::SerializationError(err.to_string())
    }
}

impl From<Error> for HyperliquidError {
    fn from(err: Error) -> Self {
        match err {
            Error::ClientRequest { message, .. } => HyperliquidError::InvalidResponse(message),
            Error::ServerRequest { message } => HyperliquidError::NetworkError(message),
            Error::ChainNotAllowed => HyperliquidError::InvalidParameter("Chain not allowed".to_string()),
            Error::InvalidSignature(msg) => HyperliquidError::SignatureError(msg),
            Error::AlloyConversion(msg) => HyperliquidError::SerializationError(msg),
            Error::SerdeJson(err) => HyperliquidError::SerializationError(err.to_string()),
            Error::ReqwestError(err) => HyperliquidError::NetworkError(err.to_string()),
            Error::Eip712(msg) => HyperliquidError::SignatureError(msg),
            Error::GenericParse(msg) => HyperliquidError::SerializationError(msg),
            Error::AssetNotFound(msg) => HyperliquidError::InvalidAsset(msg),
            Error::VaultAddressNotFound(msg) => HyperliquidError::InvalidParameter(msg),
            Error::FloatStringParse(msg) => HyperliquidError::SerializationError(msg),
            Error::GenericRequest(msg) => HyperliquidError::NetworkError(msg),
            Error::SubscriptionNotFound => HyperliquidError::WebsocketError("Subscription not found".to_string()),
            Error::WsManagerNotFound => HyperliquidError::WebsocketError("WS manager not instantiated".to_string()),
            Error::WsSend(msg) => HyperliquidError::WebsocketError(msg),
            Error::ReaderDataNotFound => HyperliquidError::WebsocketError("Reader data not found".to_string()),
            Error::GenericReader(msg) => HyperliquidError::WebsocketError(msg),
            Error::ReaderTextConversion(msg) => HyperliquidError::WebsocketError(msg),
            Error::OrderTypeNotFound => HyperliquidError::OrderError("Order type not found".to_string()),
            Error::RandGen(msg) => HyperliquidError::InternalError(msg),
            Error::PrivateKeyParse(msg) => HyperliquidError::SignatureError(msg),
            Error::UserEvents => HyperliquidError::WebsocketError("Cannot subscribe to multiple user events".to_string()),
            Error::RmpParse(msg) => HyperliquidError::SerializationError(msg),
            Error::JsonParse(msg) => HyperliquidError::SerializationError(msg),
            Error::Websocket(msg) => HyperliquidError::WebsocketError(msg),
            Error::SignatureFailure(msg) => HyperliquidError::SignatureError(msg),
            Error::AlloySignerError(msg) => HyperliquidError::SignatureError(msg),
        }
    }
}

pub(crate) type Result<T> = std::result::Result<T, HyperliquidError>;
