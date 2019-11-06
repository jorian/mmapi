use super::network::client::Client as RpcClient;
use crate::types::request;
use crate::types::response;
use crate::error::ApiError;

pub struct Client {
    rpc_client: RpcClient
}

impl Client {
    pub fn new() -> Self {
        Client {
            rpc_client: RpcClient::new()
        }
    }

    pub fn electrum(&self, userpass: &str, coin: &str) -> Result<response::Electrum, ApiError> {
        self.rpc_client.send(
            request::Electrum {
                userpass: userpass.to_string(),
                method: "electrum".to_string(),
                coin: coin.to_string(),
                servers: request::ElectrumServer::get_all(coin),
                mm2: 1
            }
        )
    }

    pub fn balance(&self, userpass: &str, coin: &str) -> Result<response::Balance, ApiError> {
        self.rpc_client.send(
            request::Balance {
                userpass: userpass.to_string(),
                method: "my_balance".to_string(),
                coin: coin.to_string()
            }
        )
    }

    pub fn enabled_coins(&self, userpass: &str) -> Result<response::EnabledCoins, ApiError> {
        self.rpc_client.send2(
            request::Generic {
                userpass: String::from(userpass),
                method: String::from("get_enabled_coins")
            }
        )
    }
}