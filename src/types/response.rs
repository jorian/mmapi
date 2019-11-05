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