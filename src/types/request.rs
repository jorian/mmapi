use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Electrum {
    pub userpass: String,
    pub method: String,
    pub coin: String,
    pub servers: Vec<ElectrumServer>,
    pub mm2: u8
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