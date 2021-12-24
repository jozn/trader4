use super::*;
use crate::base::*;
use crate::candle::{Tick, TimeSerVec};
use crate::ta::*;
use serde::{Deserialize, Serialize};
use crate::helper;


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SimpleCandle {
    pub open_time: u64, // in mill seconds
    pub open_time_str: String,
    pub close_time: u64,
    pub tick_count: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl SimpleCandle {
    // copy of aggregate_tickss_to_kline() from core/canlde.
    pub fn new(ticks: &Vec<Tick>) -> Self {
        let num = ticks.len() as u32;
        assert!(num > 0);
        let _bucket_id = 0; // From trade number

        let first = ticks.first().unwrap().clone();
        let last = ticks.last().unwrap().clone();

        let mut high = 0.;
        let mut low = f64::MAX;
        let mut _volume = 0.;

        for trade in ticks.iter() {
            if trade.price_raw > high {
                high = trade.price_raw;
            }

            if trade.price_raw < low {
                low = trade.price_raw;
            }

            _volume += trade.qty;
        }

        assert!(first.time_s < last.time_s);
        assert!(high >= low);

        Self {
            open_time: first.time_s,
            open_time_str: helper::to_time_string(first.time_s as i64),
            close_time: last.time_s,
            tick_count: num,
            open: first.price_raw,
            high: high,
            low: low,
            close: last.price_raw,
        }
    }
}
impl OHLCV for SimpleCandle {
    fn open(&self) -> f64 {
        self.open
    }

    fn high(&self) -> f64 {
        self.high
    }

    fn low(&self) -> f64 {
        self.low
    }

    fn close(&self) -> f64 {
        self.close
    }

    fn volume(&self) -> f64 {
        0.
    }
}
