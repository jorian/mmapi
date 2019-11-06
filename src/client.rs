use super::network::client::Client as RpcClient;
use crate::types::request;
use crate::types::response;
use crate::error::ApiError;

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
}