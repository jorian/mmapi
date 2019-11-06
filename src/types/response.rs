#[derive(Debug, Deserialize, PartialEq)]
pub struct Electrum {
    pub address: String,
    pub balance: String,
    pub coin: String,
    pub result: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Balance {
    pub coin: String,
    pub address: String,
    pub balance: String,
    pub locked_by_swaps: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnabledCoins(pub Vec<Coin>);

#[derive(Debug, Deserialize, PartialEq)]
pub struct Coin {
    pub address: String,
    pub ticker: String
}