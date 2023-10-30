use crate::spot::ws::message::kline::KlineIntervalTopic;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Topic {
    AccountDeals,
    AccountOrders,
    AccountUpdate,
    Deals(DealsTopic),
    Kline(KlineTopic),
    OrderBook(OrderbookTopic),
}

impl Topic {
    pub fn requires_auth(&self) -> bool {
        match self {
            Topic::AccountDeals => true,
            Topic::AccountOrders => true,
            Topic::AccountUpdate => true,
            Topic::Deals(_) => false,
            Topic::Kline(_) => false,
            Topic::OrderBook(_) => false,
        }
    }

    pub fn to_topic_subscription_string(&self) -> String {
        match self {
            Topic::AccountDeals => "spot@private.deals.v3.api".to_string(),
            Topic::AccountOrders => "spot@private.orders.v3.api".to_string(),
            Topic::AccountUpdate => "spot@private.account.v3.api".to_string(),
            Topic::Deals(deals_topic) => format!(
                "spot@public.deals.v3.api@{symbol}",
                symbol = deals_topic.symbol
            ),
            Topic::Kline(kline_topic) => format!(
                "spot@public.kline.v3.api@{symbol}@{interval}",
                symbol = kline_topic.symbol,
                interval = kline_topic.interval.as_ref()
            ),
            Topic::OrderBook(orderbook_topic) => format!(
                "spot@public.limit.depth.v3.api@{symbol}@{depth}",
                symbol = orderbook_topic.symbol,
                depth = orderbook_topic.depth
            ),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DealsTopic {
    pub symbol: String,
}

impl DealsTopic {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OrderbookTopic {
    pub symbol: String,
    pub depth: usize,
}

impl OrderbookTopic {
    pub fn new(symbol: String, depth: usize) -> Self {
        Self { symbol, depth }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KlineTopic {
    pub symbol: String,
    pub interval: KlineIntervalTopic,
}

impl KlineTopic {
    pub fn new(symbol: String, interval: KlineIntervalTopic) -> Self {
        Self { symbol, interval }
    }
}
