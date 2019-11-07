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
pub struct CancelledOrdersResult {
    pub result: CancelledOrders
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelledOrders {
    pub cancelled: Vec<String>,
    pub currently_matching: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelledOrder {
    pub result: Option<String>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KickstartCoins {
    pub result: Vec<String>
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct TradeFeeResult {
    pub result: Option<TradeFee>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TradeFee {
    pub amount: String,
    pub coin: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Orderbook {
    pub asks: Option<Vec<Ask>>,
    pub bids: Option<Vec<Bid>>,
    pub numasks: Option<u16>,
    pub numbids: Option<u16>,
    pub biddepth: Option<u16>,
    pub askdepth: Option<u16>,
    pub base: Option<String>,
    pub rel: Option<String>,
    pub timestamp: Option<u64>,
    pub netid: Option<u16>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ask {
    coin: String,
    address: String,
    price: String,
    maxvolume: f32,
    pubkey: String,
    age: u16,
    zcredits: u16
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Bid {
    coin: String,
    address: String,
    price: String,
    maxvolume: f32,
    pubkey: String,
    age: u16,
    zcredits: u16
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RecoveredFundsResult {
    pub result: Option<RecoveredFunds>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RecoveredFunds {
    pub action: String,
    pub coin: String,
    pub tx_hash: String,
    pub tx_hex: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SellResult {
    pub result: Option<Sell>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sell {
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
pub struct Withdraw {
    pub tx_hex: Option<String>,
    pub tx_hash: Option<String>,
    pub from: Option<Vec<String>>,
    pub to: Option<Vec<String>>,
    pub total_amount: Option<String>,
    pub spent_by_me: Option<String>,
    pub received_by_me: Option<String>,
    pub my_balance_change: Option<String>,
    pub block_height: Option<u32>,
    pub timestamp: Option<u64>,
    pub fee_details: Option<FeeDetails>,
    pub coin: Option<String>,
    pub internal_id: Option<String>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FeeDetails {
    amount: Option<String>,
    coin: Option<String>,
    gas: Option<u64>,
    gas_price: Option<String>,
    total_fee: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RawTxHash {
    pub tx_hash: Option<String>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SetPriceResult {
    result: Option<SetPrice>,
    error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SetPrice {
    base: String,
    rel: String,
    max_base_vol: String,
    min_base_vol: String,
    created_at: u64,
    // todo: what does the response look like?
//    matches: SetPriceMatches,
    price: String,
    started_swaps: Vec<String>,
    uuid: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConfirmationsResult {
    pub result: Option<Confirmations>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Confirmations {
    pub coin: String,
    pub confirmations: u16
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Enable {
    pub coin: Option<String>,
    pub address: Option<String>,
    pub balance: Option<String>,
    pub locked_by_swaps: Option<String>,
    pub required_confirmations: Option<u16>,
    pub result: Option<String>,
    pub error: Option<String>
}

//#[derive(Debug, Deserialize, PartialEq)]
//pub struct SetPriceMatches {
//
//}