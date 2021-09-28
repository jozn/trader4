use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Kline {
    pub open_time: u64, // in mill seconds
    pub close_time: u64,
    pub bucket: u64,
    pub tick_count: u32,
    pub kline_num: i32, // -2: from kline vsv -1: from trades sums >0 sums of klines
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl KlineId for Kline {
    fn get_kline_id(&self) -> u64 {
        self.bucket
    }
}

impl PartialEq for Kline {
    fn eq(&self, other: &Self) -> bool {
        self.open_time == other.open_time
    }
}

impl Eq for Kline {}

impl PartialOrd for Kline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.open_time.partial_cmp(&other.open_time)
    }
}

impl Ord for Kline {
    fn cmp(&self, other: &Self) -> Ordering {
        self.open_time.cmp(&other.open_time)
    }
}

impl Kline {
    pub(super) fn validate(&self) {
        assert!(self.high >= self.open);
        assert!(self.high >= self.low);
        assert!(self.high >= self.close);

        assert!(self.low <= self.open);
        assert!(self.low <= self.high);

        assert!(self.open_time <= self.close_time);
    }
}

impl OHLCV for Kline {
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
        self.volume
    }
}
