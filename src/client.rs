use super::network::client::Client as RpcClient;
use crate::types::request;
use crate::types::response;

pub struct Client {
    rpc_client: RpcClient
}

impl Client {
    pub fn new() -> Self {
        Client {
            rpc_client: RpcClient::new()
        }
    }

    pub fn electrum(&self, userpass: &str, coin: &str) -> response::Electrum {
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

    pub fn balance(&self, userpass: &str, coin: &str) -> response::Balance {
        self.rpc_client.send(
            request::Balance {
                userpass: userpass.to_string(),
                method: "my_balance".to_string(),
                coin: coin.to_string()
            }
        )
    }
}