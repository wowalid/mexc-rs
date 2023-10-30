use crate::spot::ws::message::{
    RawChannelMessage, RawChannelMessageData, RawEventChannelMessageData,
    RawEventEventChannelMessageData,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::deals::SpotDealsMessage;
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookEntry {
    pub p: Decimal, // price
    pub v: Decimal, // volume
}
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookData {
    pub asks: Vec<OrderBookEntry>,
    pub bids: Vec<OrderBookEntry>,
}
#[derive(Debug)]
pub struct SpotDepthOrderbookMessage {
    pub symbol: String,
    pub orderbook: OrderBookData,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum ChannelMessageToSpotDealsMessageError {
    #[error("No depth orderbook message")]
    NoDepthOrderbookMessage,
}

pub(crate) fn channel_message_to_orderbook_depth_message(
    channel_message: &RawChannelMessage,
) -> Result<SpotDepthOrderbookMessage, ChannelMessageToSpotDealsMessageError> {
    let Some(symbol) = &channel_message.symbol else {
        return Err(ChannelMessageToSpotDealsMessageError::NoDepthOrderbookMessage);
    };

    let RawChannelMessageData::OrderBook(orderbook) = &channel_message.data else {
        return Err(ChannelMessageToSpotDealsMessageError::NoDepthOrderbookMessage);
    };

    let message = SpotDepthOrderbookMessage {
        symbol: symbol.to_string(),
        orderbook: orderbook.clone(),
        timestamp: channel_message.timestamp.clone(),
    };
    Ok(message)
}
