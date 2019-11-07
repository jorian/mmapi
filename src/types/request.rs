use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Electrum {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub servers: Vec<ElectrumServer>,
    pub mm2: u8,
    pub tx_history: bool
}

#[derive(Debug, Serialize)]
pub struct ElectrumServer {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cert_verification: Option<bool>
}

impl ElectrumServer {
    pub fn get_random(coin: &str) -> Self {
        unimplemented!()
    }

    /// Fetches all available electrums from coins repo.
    pub fn get_all(coin: &str) -> Vec<Self> {
        let response = reqwest::get(&format!("https://raw.githubusercontent.com/jorian/coins/master/electrums/{}", coin));
        let mut data = response.expect("unable to fetch electrum servers"); // todo throw dialog when error
        let data: Value = data.json().expect("error while converting coins/electrum response to json"); // todo throw error dialog when error
        let array = data.as_array().expect("something went wrong while converting to array; are there any electrums at all?"); // todo error

        let mut servers = vec![];
        for a in array {
            servers.push(ElectrumServer {
                url: a["url"].as_str().unwrap().to_string(),
                protocol: {
                    match a["tcp"].as_str() {
                        None => None,
                        Some(tcp) => Some(tcp.to_string())
                    }
                },
                disable_cert_verification: {
                    match a["disable_cert_verification"].as_str() {
                        None => None,
                        Some(cert) => Some(cert.parse().unwrap())
                    }
                },
            });
        };

        servers
    }
}

#[derive(Debug, Serialize)]
pub struct Buy {
    pub userpass: String,
    pub method: String,
    pub base: String,
    pub rel: String,
    pub price: String,
    pub volume: String
}

#[derive(Debug, Serialize)]
pub struct CancelAllOrders {
    pub userpass: String,
    pub method: String,
    pub cancel_by: CancelBy
}

#[derive(Debug, Serialize)]
pub struct CancelBy {
    #[serde(rename = "type")]
    type_: String, // actually an enum (All, Pair, Coin)
    data: CancelByData
}

// either base AND rel need to be used, or just ticker.
#[derive(Debug, Serialize)]
pub struct CancelByData {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<String>
}

#[derive(Debug, Serialize)]
pub struct CancelOrder {
    pub userpass: String,
    pub method: String,
    pub uuid: String
}

#[derive(Debug, Serialize)]
pub struct Balance {
    pub userpass: String,
    pub method: String,
    pub coin: String
}

#[derive(Debug, Serialize)]
pub struct Generic {
    pub userpass: String,
    pub method: String
}

#[derive(Debug, Serialize)]
pub struct Disable {
    pub userpass: String,
    pub method: String,
    pub coin: String
}

#[derive(Debug, Serialize)]
pub struct TradeFee {
    pub userpass: String,
    pub method: String,
    pub coin: String
}

#[derive(Debug, Serialize)]
pub struct Orderbook {
    pub userpass: String,
    pub method: String,
    pub base: String,
    pub rel: String
}

#[derive(Debug, Serialize)]
pub struct RecoverFunds {
    pub userpass: String,
    pub method: String,
    pub params: RecoveryUuid
}

#[derive(Debug, Serialize)]
pub struct RecoveryUuid {
    pub uuid: String
}

#[derive(Debug, Serialize)]
pub struct Sell {
    pub userpass: String,
    pub method: String,
    pub base: String,
    pub rel: String,
    pub price: String,
    pub volume: String
}

#[derive(Debug, Serialize)]
pub struct Withdraw {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub to: String,
    pub amount: String,
    pub max: bool,
    pub fee: Option<WithdrawFee>
}

#[derive(Debug, Serialize)]
pub struct WithdrawFee {
    #[serde(rename = "type")]
    pub type_: String,
    pub amount: String,
    pub gas_price: String,
    pub gas: u32
}

#[derive(Debug, Serialize)]
pub struct RawTransaction {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub tx_hex: String,
}

#[derive(Debug, Serialize)]
pub struct SetPrice {
    pub userpass: String,
    pub method: String,
    pub base: String,
    pub rel: String,
    pub price: String,
    pub volume: String,
    pub max: bool,
    pub cancel_previous: bool
}

#[derive(Debug, Serialize)]
pub struct Confirmations {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub confirmations: u16
}

#[derive(Debug, Serialize)]
pub struct Enable {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub urls: Option<Vec<String>>,
    pub swap_contract_address: Option<String>,
    pub gas_station_url: Option<String>,
    pub mm2: u8,
    pub tx_history: bool
}