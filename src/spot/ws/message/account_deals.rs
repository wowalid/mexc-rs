use crate::spot::v3::enums::OrderSide;
use crate::spot::ws::message::{RawChannelMessage, RawChannelMessageData};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct RawAccountDealsData {
    pub S: u8,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub T: DateTime<Utc>,
    pub c: String,
    pub i: String,
    pub m: u8,
    pub p: Decimal,
    pub st: u8,
    pub t: String,
    pub v: Decimal,
    pub a: Decimal,
    pub n: Decimal,
    pub N: String,
}

#[derive(Debug)]
pub struct AccountDealsMessage {
    pub asset: String,
    pub trade_type: OrderSide,
    pub trade_time: DateTime<Utc>,
    pub client_order_id: String,
    pub order_id: String,
    pub is_maker: bool,
    pub price: Decimal,
    pub is_self_trade: bool,
    pub trade_id: String,
    pub quantity: Decimal,
    pub deals_amount: Decimal,
    pub commission_fee: Decimal,
    pub commission_asset: String,
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ChannelMessageToAccountDealsMessageError {
    #[error("Invalid channel message")]
    InvalidChannelMessage,
}

pub(crate) fn channel_message_to_account_deals_message(
    message: &RawChannelMessage,
) -> Result<AccountDealsMessage, ChannelMessageToAccountDealsMessageError> {
    let RawChannelMessageData::AccountDeals(account_deals_data) = &message.data else {
        return Err(ChannelMessageToAccountDealsMessageError::InvalidChannelMessage);
    };
    let Some(asset) = &message.symbol else {
        return Err(ChannelMessageToAccountDealsMessageError::InvalidChannelMessage);
    };

    let message = AccountDealsMessage {
        asset: asset.clone(),
        trade_type: if account_deals_data.S == 1 {
            OrderSide::Buy
        } else if account_deals_data.S == 2 {
            OrderSide::Sell
        } else {
            return Err(ChannelMessageToAccountDealsMessageError::InvalidChannelMessage);
        },
        trade_time: account_deals_data.T,
        client_order_id: account_deals_data.c.clone(),
        order_id: account_deals_data.i.clone(),
        is_maker: account_deals_data.m == 1,
        price: account_deals_data.p,
        is_self_trade: account_deals_data.st == 1,
        trade_id: account_deals_data.t.clone(),
        quantity: account_deals_data.v,
        deals_amount: account_deals_data.a,
        commission_fee: account_deals_data.n,
        commission_asset: account_deals_data.N.clone(),
        event_time: message.timestamp,
    };

    Ok(message)
}
