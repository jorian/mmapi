#[derive(Debug, Deserialize, PartialEq)]
pub struct BuyResult {
    pub result: Option<Buy>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Buy {
    pub action: String,
    pub base: String,
    pub base_amount: String,
    pub rel: String,
    pub rel_amount: String,
    pub method: String,
    pub dest_pub_key: String,
    pub sender_pubkey: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Electrum {
    pub address: Option<String>,
    pub balance: Option<String>,
    pub coin: Option<String>,
    pub result: Option<String>,
    pub locked_by_swaps: Option<String>,
    pub required_confirmations: Option<u32>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Balance {
    pub coin: Option<String>,
    pub address: Option<String>,
    pub balance: Option<String>,
    pub locked_by_swaps: Option<String>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnabledCoins {
    pub result: Vec<EnabledCoin>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnabledCoin {
    pub address: String,
    pub ticker: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelledOrder {
    pub result: Option<String>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DisabledCoinResult {
    pub result: Option<DisabledCoin>,
    pub error: Option<String>,
    pub swaps: Option<Vec<String>>,
    pub orders: Option<DisabledCoinOrders>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DisabledCoin {
    pub cancelled_orders: Vec<String>,
    pub coin: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DisabledCoinOrders {
    pub matching: Vec<String>,
    pub cancelled: Vec<String>
}