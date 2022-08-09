use crate::configs::assets::Pair;

#[derive(Debug)]
pub struct TransTickData {
    pub timestamp: i64,
    pub bid_price: i64,
    pub ask_price: i64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct BTickData {
    #[serde(skip)]
    pub pair: Pair,
    pub date_str: String,
    pub timestamp_sec: i64,
    pub timestamp: i64,
    pub bid_price: f64, // low price
    pub ask_price: f64, // high price - for long buy erntry -
}

impl BTickData {
    // dep
    pub fn get_price(&self) -> f64 {
        (self.bid_price + self.ask_price) / 2.
    }

    pub fn get_spread_pip(&self, pair: &Pair) -> f64 {
        (self.ask_price - self.bid_price) * pair.get_pip_multi()
    }

    pub fn to_fast_bin(&self) -> TickBinFast {
        TickBinFast {
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct TickBinFast {
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}

impl TickBinFast {
    pub fn to_tick(&self, pair: &Pair) -> BTickData {
        BTickData {
            pair: pair.clone(),
            date_str: "".to_string(),
            timestamp_sec: self.timestamp / 1000,
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price,
            // ask_price: self.bid_price, // todo: remove temp testing no spread
        }
    }
}
