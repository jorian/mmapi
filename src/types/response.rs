use std::collections::HashMap;

type UUID = String;

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

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrdersResult {
    result: Option<Orders>,
    error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Orders {
    maker_orders: HashMap<UUID, MakerOrder>,
    taker_orders: HashMap<UUID, TakerOrder>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderStatus {
    order: OrderType,
    // the following field is somewhat redundant as the enum above defines the type or order
    #[serde(rename = "type")]
    order_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OrderType {
    MakerOrder(MakerOrder),
    TakerOrder(TakerOrder)
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MakerOrder {
    available_amount: String,
    base: String,
    cancellable: bool,
    created_at: u64,
    matches: HashMap<UUID, Matches>,
    max_base_vol: String,
    min_base_vol: String,
    price: String,
    rel: String,
    started_swaps: Vec<Swap>,
    uuid: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TakerOrder {
    cancellable: bool,
    created_at: u64,
    matches: HashMap<UUID, Matches>,
    request: MatchRequest,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Matches {
    connect: MatchDetails,
    connected: MatchDetails,
    // request is only used in MakerOrders
    request: Option<MatchRequest>,
    last_updated: u64,
    reserved: MatchReserved
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MatchDetails {
    dest_pub_key: String,
    maker_order_uuid: String,
    method: String,
    sender_pubkey: String,
    taker_order_uuid: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MatchRequest {
    action: String,
    base: String,
    base_amount: String,
    dest_pub_key: String,
    method: String,
    rel: String,
    rel_amount: String,
    sender_pubkey: String,
    uuid: UUID,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MatchReserved {
    base: String,
    base_amount: String,
    dest_pub_key: String,
    maker_order_uuid: UUID,
    method: String,
    rel: String,
    rel_amount: String,
    sender_pubkey: String,
    taker_order_uuid: UUID
}

/////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize, PartialEq)]
pub struct RecentSwapsResult {
    result: Option<RecentSwaps>,
    error: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RecentSwaps {
    from_uuid: Option<UUID>,
    limit: u32,
    skipped: u32,
    swaps: Vec<Swap>,
    total: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Swap {
    error_events: Option<Vec<String>>,
    events: Vec<SwapEvent>,
    gui: String,
    maker_amount: String,
    maker_coin: String,
    mm_version: String,
    my_info: SwapInfo,
    recoverable: bool,
    success_events: Vec<String>,
    taker_amount: String,
    taker_coin: String,
    #[serde(rename = "type")]
    swap_type: String,
    uuid: UUID,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SuccessEvents(Vec<String>);

#[derive(Debug, Deserialize, PartialEq)]
pub struct SwapInfo {
    my_amount: String,
    my_coin: String,
    other_amount: String,
    other_coin: String,
    started_at: u64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SwapEvent {
    event: SwapEventDetails,
    timestamp: u64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SwapEventDetails {
    data: Option<SwapEventData>,
    #[serde(rename = "type")]
    event_type: SwapEventType, // TODO make this an Enum
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SwapEventData {
    error: Option<String>,
    block_height: Option<u32>,
    coin: Option<String>,
    fee_details: Option<FeeDetails>,
    from: Option<Vec<String>>,
    internal_id: Option<String>,
    lock_duration: Option<u64>,
    maker_amount: Option<String>,
    maker_coin: Option<String>,
    maker_coin_start_block: Option<u32>,
    maker_payment_confirmations: Option<u32>,
    maker_payment_lock: Option<u64>,
    my_balance_change: Option<String>,
    my_persistent_pub: Option<String>,
    received_by_me: Option<String>,
    secret: Option<String>,
    secret_hash: Option<String>,
    spent_by_me: Option<String>,
    started_at: Option<u64>,
    taker: Option<String>,
    taker_amount: Option<String>,
    taker_coin: Option<String>,
    taker_coin_start_block: Option<u32>,
    taker_payment_confirmations: Option<u16>,
    taker_payment_locktime: Option<u64>,
    taker_pubkey: Option<String>,
    timestamp: Option<u64>,
    to: Option<Vec<String>>,
    total_amount: Option<String>,
    tx_hash: Option<String>,
    tx_hex: Option<String>,
    uuid: Option<UUID>,
}

#[derive(Debug, Deserialize, PartialEq)]
enum SwapEventType {
    Started,
    Negotiated,
    TakerFeeValidated,
    MakerPaymentSent,
    TakerPaymentReceived,
    TakerPaymentWaitConfirmStarted,
    TakerPaymentValidatedAndConfirmed,
    TakerPaymentSpent,
    Finished,
    StartFailed,
    NegotiateFailed,
    TakerFeeValidateFailed,
    MakerPaymentTransactionFailed,
    MakerPaymentDataSendFailed,
    TakerPaymentValidateFailed,
    TakerPaymentSpendFailed,
    MakerPaymentRefunded,
    MakerPaymentRefundFailed,
}

//////
#[derive(Debug, Deserialize, PartialEq)]
pub struct SwapStatusResult {
    result: Option<Swap>,
    error: Option<String>,
}