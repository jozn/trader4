#[derive(Debug)]
pub struct TransTickData {
    pub timestamp: i64,
    pub bid_price: i64,
    pub ask_price: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct BTickData {
    pub date_str: String,
    pub timestamp_sec: i64,
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}