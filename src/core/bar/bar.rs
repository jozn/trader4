use prost::Message;
use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;
use crate::collector::row_data::BTickData;
use crate::{helper, ta};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bar {
    pub seq: i32,
    #[serde(skip)]
    pub open_time: i64, // in mill seconds
    #[serde(skip)]
    pub close_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    #[serde(skip)]
    pub volume: f64,
    pub ticks: u32,

    #[serde(rename = "open_time")]
    pub open_time_str: String,
    pub duration: String,

    pub pip_hl: f64,
    pub pip_co: f64,

    pub spreed_min: f64,
    pub spreed_max: f64,

    #[serde(skip)]
    pub ta: BarTA,
}

impl Bar {
    pub fn new(ticks: &Vec<BTickData>) -> Bar {
        let counts = ticks.len() as u32;
        assert!(counts > 0);

        let first = ticks.first().unwrap().clone();
        let last = ticks.last().unwrap().clone();

        let mut high = 0.;
        let mut low = f64::MAX;
        let mut volume = 0.;

        for tick in ticks.iter() {
            let price = tick.get_price();
            if price > high {
                high = price;
            }

            if price < low {
                low = price;
            }

            // volume += trade.;
        }

        let open = first.get_price();
        let close = last.get_price();

        let mut bar = Bar {
            seq: 0,
            open_time: first.timestamp,
            close_time: last.timestamp,
            open,
            high,
            low,
            close,
            volume,
            ticks: counts,

            open_time_str: helper::to_time_string(first.timestamp_sec),
            duration: helper::to_duration(first.timestamp_sec - last.timestamp_sec),
            pip_hl: (high - low) * 10_000.,
            pip_co: (close - open).abs() * 10_000.,
            spreed_min: 0.0,
            spreed_max: 0.0,

            ta: Default::default(),
        };

        bar.spreed_min = f64::MAX;
        for t in ticks {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > bar.spreed_max {
                bar.spreed_max = spread;
            }
            if spread < bar.spreed_min {
                bar.spreed_min = spread;
            }
        }
        bar
    }

    pub(super) fn validate(&self) {
        assert!(self.high >= self.open);
        assert!(self.high >= self.low);
        assert!(self.high >= self.close);

        assert!(self.low <= self.open);
        assert!(self.low <= self.high);

        assert!(self.open_time <= self.close_time);
    }
    pub fn to_json_out(&self) -> JsonOHLC {
        JsonOHLC {
            date: self.open_time / 1000,
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
        }
    }
}

impl OHLCV for Bar {
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
