use super::network::client::Client as RpcClient;
use crate::types::request;
use crate::types::response;
use crate::error::ApiError;
use crate::types::request::{RecoveryUuid, WithdrawFee};

pub struct Client {
    rpc_client: RpcClient,
    userpass: String
}

impl Client {
    pub fn new(userpass: &str) -> Self {
        Client {
            rpc_client: RpcClient::new(),
            userpass: String::from(userpass)
        }
    }

    pub fn buy(&self, base: &str, rel: &str, price: f64, volume: f64) -> Result<response::BuyResult, ApiError> {
        self.rpc_client.send(request::Buy {
            userpass: String::from(&self.userpass),
            method: "buy".to_string(),
            base: base.to_string(),
            rel: rel.to_string(),
            price: price.to_string(),
            volume: volume.to_string()
        })
    }

    pub fn electrum(&self, coin: &str, tx_history: bool) -> Result<response::Electrum, ApiError> {
        self.rpc_client.send(request::Electrum {
            userpass: String::from(&self.userpass),
            method: "electrum".to_string(),
            coin: coin.to_string(),
            servers: request::ElectrumServer::get_all(coin),
            mm2: 1,
            tx_history
        })
    }

    pub fn balance(&self, coin: &str) -> Result<response::Balance, ApiError> {
        self.rpc_client.send(request::Balance {
            userpass: String::from(&self.userpass),
            method: "my_balance".to_string(),
            coin: coin.to_string()
        })
    }

    pub fn enabled_coins(&self) -> Result<response::EnabledCoins, ApiError> {
        self.rpc_client.send(request::Generic {
            userpass: String::from(&self.userpass),
            method: String::from("get_enabled_coins")
        })
    }

    pub fn cancel_order(&self, uuid: &str) -> Result<response::CancelledOrder, ApiError> {
        self.rpc_client.send(request::CancelOrder {
            userpass: String::from(&self.userpass),
            method: String::from("cancel_order"),
            uuid: uuid.to_string()
        })
    }

    pub fn coins_needed_for_kick_start(&self) -> Result<response::KickstartCoins, ApiError> {
        self.rpc_client.send(request::Generic {
            userpass: String::from(&self.userpass),
            method: String::from("coins_needed_for_kick_start")
        })
    }

    pub fn trade_fee(&self, coin: &str) -> Result<response::TradeFeeResult, ApiError> {
        self.rpc_client.send(request::TradeFee {
            userpass: String::from(&self.userpass),
            method: String::from("get_trade_fee"),
            coin: coin.to_string()
        })
    }

    pub fn orderbook(&self, base: &str, rel: &str) -> Result<response::Orderbook, ApiError> {
        self.rpc_client.send(request::Orderbook {
            userpass: String::from(&self.userpass),
            method: "orderbook".to_string(),
            base: base.to_string(),
            rel: rel.to_string()
        })
    }

    pub fn recover_funds(&self, uuid: &str) -> Result<response::RecoveredFundsResult, ApiError> {
        self.rpc_client.send(request::RecoverFunds {
            userpass: String::from(&self.userpass),
            method: "recover_funds_of_swap".to_string(),
            params: RecoveryUuid {
                uuid: String::from(uuid)
            }
        })
    }

    // todo: price and volume as rational / f64
    pub fn sell(&self, base: &str, rel: &str, price: f64, volume: f64) -> Result<response::SellResult, ApiError> {
        self.rpc_client.send(request::Sell {
            userpass: String::from(&self.userpass),
            method: "buy".to_string(),
            base: base.to_string(),
            rel: rel.to_string(),
            price: price.to_string(),
            volume: volume.to_string()
        })
    }

    pub fn withdraw(&self, coin: &str, to: &str, amount: f64, max: bool, fee: Option<WithdrawFee>) -> Result<response::Withdraw, ApiError> {
        self.rpc_client.send(request::Withdraw {
            userpass: String::from(&self.userpass),
            method: "withdraw".to_string(),
            coin: coin.to_string(),
            to: to.to_string(),
            amount: amount.to_string(),
            max,
            fee
        })
    }

    pub fn send_raw_transaction(&self, coin: &str, hex: &str) -> Result<response::RawTxHash, ApiError> {
        self.rpc_client.send(request::RawTransaction {
            userpass: String::from(&self.userpass),
            method: "send_raw_transaction".to_string(),
            coin: coin.to_string(),
            tx_hex: hex.to_string()
        })
    }

    pub fn set_price(&self, base: &str, rel: &str, price: f64, volume: f64, max: bool, cancel_previous: bool) -> Result<response::SetPriceResult, ApiError> {
        self.rpc_client.send(request::SetPrice {
            userpass: String::from(&self.userpass),
            method: "setprice".to_string(),
            base: base.to_string(),
            rel: rel.to_string(),
            price: price.to_string(),
            volume: volume.to_string(),
            max,
            cancel_previous,
        })
    }
}