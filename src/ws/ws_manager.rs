use crate::errors::Error;
use crate::ws::message_types::{
    ActiveAssetCtx, AllMids, Candle, L2Book, Notification, OrderUpdates,
    Trades, User, UserFills, UserFundings, UserNonFundingLedgerUpdates, WebData2,
};
use alloy_primitives::Address;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc::UnboundedSender, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::{self, protocol}, MaybeTlsStream, WebSocketStream};
use std::collections::HashMap;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::ops::DerefMut;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, time::Duration};
use tokio::spawn;
use tokio::time;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WsError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Subscription error: {0}")]
    Subscription(String),
    #[error("JSON parse error: {0}")]
    JsonParse(String),
}

impl From<Error> for WsError {
    fn from(err: Error) -> Self {
        match err {
            Error::JsonParse(msg) => WsError::JsonParse(msg),
            _ => WsError::Connection(err.to_string()),
        }
    }
}

type Result<T> = std::result::Result<T, WsError>;

#[derive(Debug)]
struct SubscriptionData {
    sending_channel: UnboundedSender<Message>,
    subscription_id: u32,
    id: String,
}

#[derive(Debug, Clone)]
pub struct WsManager {
    pub url: String,
    pub subscriptions: Vec<Subscription>,
    stop_flag: Arc<AtomicBool>,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, protocol::Message>>>,
    subscriptions_map: Arc<Mutex<HashMap<String, Vec<SubscriptionData>>>>,
    subscription_id: u32,
    subscription_identifiers: HashMap<u32, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Subscription {
    AllMids,
    Notification { user: Address },
    WebData2 { user: Address },
    Candle { coin: String, interval: String },
    L2Book { coin: String },
    Trades { coin: String },
    OrderUpdates { user: Address },
    UserEvents { user: Address },
    UserFills { user: Address },
    UserFundings { user: Address },
    UserNonFundingLedgerUpdates { user: Address },
    ActiveAssetCtx { coin: String },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "channel")]
#[serde(rename_all = "camelCase")]
pub enum Message {
    NoData,
    HyperliquidError(String),
    AllMids(AllMids),
    Trades(Trades),
    L2Book(L2Book),
    User(User),
    UserFills(UserFills),
    Candle(Candle),
    SubscriptionResponse,
    OrderUpdates(OrderUpdates),
    UserFundings(UserFundings),
    UserNonFundingLedgerUpdates(UserNonFundingLedgerUpdates),
    Notification(Notification),
    WebData2(WebData2),
    ActiveAssetCtx(ActiveAssetCtx),
    Pong,
}

#[derive(Serialize)]
pub(crate) struct SubscriptionSendData<'a> {
    method: &'static str,
    subscription: &'a serde_json::Value,
}

#[derive(Serialize)]
pub(crate) struct Ping {
    method: &'static str,
}

impl WsManager {
    const SEND_PING_INTERVAL: u64 = 50;

    pub(crate) async fn new(url: String, reconnect: bool) -> Result<WsManager> {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let url = Arc::new(url);

        let (writer, mut reader) = Self::connect(&url).await?.split();
        let writer = Arc::new(Mutex::new(writer));

        let subscriptions_map = Arc::new(Mutex::new(HashMap::<String, Vec<SubscriptionData>>::new()));
        let subscriptions_copy = Arc::clone(&subscriptions_map);

        {
            let writer = writer.clone();
            let stop_flag = Arc::clone(&stop_flag);
            let url = Arc::clone(&url);
            let reader_fut = async move {
                while !stop_flag.load(Ordering::Relaxed) {
                    if let Some(data) = reader.next().await {
                        if let Err(err) =
                            WsManager::parse_and_send_data(data, &subscriptions_copy).await
                        {
                            error!("Error processing data received by WsManager reader: {err}");
                        }
                    } else {
                        warn!("WsManager disconnected");
                        if let Err(err) = WsManager::send_to_all_subscriptions(
                            &subscriptions_copy,
                            Message::NoData,
                        )
                        .await
                        {
                            warn!("Error sending disconnection notification err={err}");
                        }
                        if reconnect {
                            // Always sleep for 1 second before attempting to reconnect so it does not spin during reconnecting. This could be enhanced with exponential backoff.
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            info!("WsManager attempting to reconnect");
                            match Self::connect(&url).await {
                                Ok(ws) => {
                                    let (new_writer, new_reader) = ws.split();
                                    reader = new_reader;
                                    let mut writer_guard = writer.lock().await;
                                    *writer_guard = new_writer;
                                    for (identifier, v) in subscriptions_copy.lock().await.iter() {
                                        // TODO should these special keys be removed and instead use the simpler direct identifier mapping?
                                        if identifier.eq("userEvents")
                                            || identifier.eq("orderUpdates")
                                        {
                                            for subscription_data in v {
                                                if let Err(err) = Self::subscribe(
                                                    writer_guard.deref_mut(),
                                                    &subscription_data.id,
                                                )
                                                .await
                                                {
                                                    error!(
                                                        "Could not resubscribe {identifier}: {err}"
                                                    );
                                                }
                                            }
                                        } else if let Err(err) =
                                            Self::subscribe(writer_guard.deref_mut(), identifier)
                                                .await
                                        {
                                            error!("Could not resubscribe correctly {identifier}: {err}");
                                        }
                                    }
                                    info!("WsManager reconnect finished");
                                }
                                Err(err) => error!("Could not connect to websocket {err}"),
                            }
                        } else {
                            error!("WsManager reconnection disabled. Will not reconnect and exiting reader task.");
                            break;
                        }
                    }
                }
                warn!("ws message reader task stopped");
            };
            spawn(reader_fut);
        }

        {
            let stop_flag = Arc::clone(&stop_flag);
            let writer = Arc::clone(&writer);
            let ping_fut = async move {
                while !stop_flag.load(Ordering::Relaxed) {
                    match serde_json::to_string(&Ping { method: "ping" }) {
                        Ok(payload) => {
                            let mut writer = writer.lock().await;
                            if let Err(err) = writer.send(protocol::Message::Text(payload)).await {
                                error!("Error pinging server: {err}")
                            }
                        }
                        Err(err) => error!("Error serializing ping message: {err}"),
                    }
                    time::sleep(Duration::from_secs(Self::SEND_PING_INTERVAL)).await;
                }
                warn!("ws ping task stopped");
            };
            spawn(ping_fut);
        }

        Ok(WsManager {
            url: (*url).clone(),
            subscriptions: Vec::new(),
            stop_flag,
            writer,
            subscriptions_map,
            subscription_id: 0,
            subscription_identifiers: HashMap::new(),
        })
    }

    async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        Ok(connect_async(url)
            .await
            .map_err(|e| Error::Websocket(e.to_string()))?
            .0)
    }

    fn get_identifier(message: &Message) -> Result<String> {
        match message {
            Message::AllMids(_) => serde_json::to_string(&Subscription::AllMids)
                .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::User(_) => Ok("userEvents".to_string()),
            Message::UserFills(fills) => serde_json::to_string(&Subscription::UserFills {
                user: fills.data.user,
            })
            .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::Trades(trades) => {
                if trades.data.is_empty() {
                    Ok(String::default())
                } else {
                    serde_json::to_string(&Subscription::Trades {
                        coin: trades.data[0].coin.clone(),
                    })
                    .map_err(|e| WsError::JsonParse(e.to_string()))
                }
            }
            Message::L2Book(l2_book) => serde_json::to_string(&Subscription::L2Book {
                coin: l2_book.data.coin.clone(),
            })
            .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::Candle(candle) => serde_json::to_string(&Subscription::Candle {
                coin: candle.data.coin.clone(),
                interval: candle.data.interval.clone(),
            })
            .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::OrderUpdates(_) => Ok("orderUpdates".to_string()),
            Message::UserFundings(user_fundings) => serde_json::to_string(&Subscription::UserFundings {
                user: user_fundings.data.user,
            })
            .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::UserNonFundingLedgerUpdates(user_non_funding_ledger_updates) => {
                serde_json::to_string(&Subscription::UserNonFundingLedgerUpdates {
                    user: user_non_funding_ledger_updates.data.user,
                })
                .map_err(|e| WsError::JsonParse(e.to_string()))
            }
            Message::Notification(_) => Ok("notification".to_string()),
            Message::WebData2(web_data2) => serde_json::to_string(&Subscription::WebData2 {
                user: web_data2.data.user,
            })
            .map_err(|e| WsError::JsonParse(e.to_string())),
            Message::ActiveAssetCtx(active_asset_ctx) => {
                serde_json::to_string(&Subscription::ActiveAssetCtx {
                    coin: active_asset_ctx.data.coin.clone(),
                })
                .map_err(|e| WsError::JsonParse(e.to_string()))
            }
            Message::SubscriptionResponse | Message::Pong => Ok(String::default()),
            Message::NoData => Ok("".to_string()),
            Message::HyperliquidError(_) => Ok(String::default()),
        }
    }

    async fn parse_and_send_data(
        data: std::result::Result<protocol::Message, tungstenite::Error>,
        subscriptions: &Arc<Mutex<HashMap<String, Vec<SubscriptionData>>>>,
    ) -> Result<()> {
        match data {
            Ok(data) => match data.into_text() {
                Ok(data) => {
                    if !data.starts_with('{') {
                        return Ok(());
                    }
                    let message = serde_json::from_str::<Message>(&data)
                        .map_err(|e| WsError::JsonParse(e.to_string()))?;
                    let identifier = WsManager::get_identifier(&message)?;
                    if identifier.is_empty() {
                        return Ok(());
                    }

                    let mut subscriptions = subscriptions.lock().await;
                    if let Some(subscription_datas) = subscriptions.get_mut(&identifier) {
                        for subscription_data in subscription_datas {
                            if let Err(e) = subscription_data.sending_channel.send(message.clone()) {
                                return Err(WsError::Connection(e.to_string()));
                            }
                        }
                    }
                    Ok(())
                }
                Err(err) => {
                    let error = WsError::Connection(err.to_string());
                    Ok(WsManager::send_to_all_subscriptions(
                        subscriptions,
                        Message::HyperliquidError(error.to_string()),
                    )
                    .await?)
                }
            },
            Err(err) => {
                let error = WsError::WebSocket(err);
                Ok(WsManager::send_to_all_subscriptions(
                    subscriptions,
                    Message::HyperliquidError(error.to_string()),
                )
                .await?)
            }
        }
    }

    async fn send_to_all_subscriptions(
        subscriptions: &Arc<Mutex<HashMap<String, Vec<SubscriptionData>>>>,
        message: Message,
    ) -> Result<()> {
        let mut subscriptions = subscriptions.lock().await;
        for subscription_datas in subscriptions.values_mut() {
            for subscription_data in subscription_datas {
                if let Err(e) = subscription_data.sending_channel.send(message.clone()) {
                    return Err(WsError::Connection(e.to_string()));
                }
            }
        }
        Ok(())
    }

    async fn send_subscription_data(
        method: &'static str,
        writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, protocol::Message>,
        identifier: &str,
    ) -> Result<()> {
        let payload = serde_json::to_string(&SubscriptionSendData {
            method,
            subscription: &serde_json::from_str::<serde_json::Value>(identifier)
                .map_err(|e| WsError::JsonParse(e.to_string()))?,
        })
        .map_err(|e| WsError::JsonParse(e.to_string()))?;

        writer
            .send(protocol::Message::Text(payload))
            .await
            .map_err(|e| WsError::WebSocket(e))?;
        Ok(())
    }

    async fn subscribe(
        writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, protocol::Message>,
        identifier: &str,
    ) -> Result<()> {
        Self::send_subscription_data("subscribe", writer, identifier).await
    }

    async fn unsubscribe(
        writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, protocol::Message>,
        identifier: &str,
    ) -> Result<()> {
        Self::send_subscription_data("unsubscribe", writer, identifier).await
    }

    pub(crate) async fn add_subscription(
        &mut self,
        identifier: String,
        sending_channel: UnboundedSender<Message>,
    ) -> Result<u32> {
        let mut subscriptions = self.subscriptions_map.lock().await;

        let identifier_entry = if let Subscription::UserEvents { user: _ } =
            serde_json::from_str::<Subscription>(&identifier)
                .map_err(|e| WsError::JsonParse(e.to_string()))?
        {
            "userEvents".to_string()
        } else if let Subscription::OrderUpdates { user: _ } =
            serde_json::from_str::<Subscription>(&identifier)
                .map_err(|e| WsError::JsonParse(e.to_string()))?
        {
            "orderUpdates".to_string()
        } else {
            identifier.clone()
        };
        let subscriptions = subscriptions
            .entry(identifier_entry.clone())
            .or_insert(Vec::new());

        if !subscriptions.is_empty() && identifier_entry.eq("userEvents") {
            return Err(WsError::Subscription("User events already subscribed".to_string()));
        }

        if subscriptions.is_empty() {
            Self::subscribe(self.writer.lock().await.borrow_mut(), identifier.as_str()).await?;
        }

        let subscription_id = self.subscription_id;
        self.subscription_identifiers
            .insert(subscription_id, identifier.clone());
        subscriptions.push(SubscriptionData {
            sending_channel,
            subscription_id,
            id: identifier,
        });

        self.subscription_id += 1;
        Ok(subscription_id)
    }

    pub(crate) async fn remove_subscription(&mut self, subscription_id: u32) -> Result<()> {
        let identifier = self
            .subscription_identifiers
            .get(&subscription_id)
            .ok_or(Error::SubscriptionNotFound)?
            .clone();

        let identifier_entry = if let Subscription::UserEvents { user: _ } =
            serde_json::from_str::<Subscription>(&identifier)
                .map_err(|e| Error::JsonParse(e.to_string()))?
        {
            "userEvents".to_string()
        } else if let Subscription::OrderUpdates { user: _ } =
            serde_json::from_str::<Subscription>(&identifier)
                .map_err(|e| Error::JsonParse(e.to_string()))?
        {
            "orderUpdates".to_string()
        } else {
            identifier.clone()
        };

        self.subscription_identifiers.remove(&subscription_id);

        let mut subscriptions = self.subscriptions_map.lock().await;

        let subscriptions = subscriptions
            .get_mut(&identifier_entry)
            .ok_or(Error::SubscriptionNotFound)?;
        let index = subscriptions
            .iter()
            .position(|subscription_data| subscription_data.subscription_id == subscription_id)
            .ok_or(Error::SubscriptionNotFound)?;
        subscriptions.remove(index);

        if subscriptions.is_empty() {
            Self::unsubscribe(self.writer.lock().await.borrow_mut(), identifier.as_str()).await?;
        }
        Ok(())
    }
}

impl Drop for WsManager {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
