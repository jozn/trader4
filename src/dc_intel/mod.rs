use crate::base::*;
use crate::candle::{Tick, TimeSerVec};
use crate::ta::*;
use serde::{Deserialize, Serialize};

pub type TResult<T> = std::result::Result<T, TErr>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TErr {
    EmptyTradesErr,
}

pub struct DCParent {
    pub frame_id: u64, // For next frame id
    pub frames: Vec<FrameMem>,

    // TA holders
    pub med_high: Maximum,
    pub med_low: Minimum,
    pub big_high: Maximum,
    pub big_low: Minimum,
    pub dc_med: DC,
    pub dc_big: DC,

    pub ma: EMA,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameMemConfig {
    pub small_tick: u64,
    pub medium_tick: u64,
    pub big_tick: u64,
    pub vel1_period: u64,
    pub vel2_period: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameMem {
    pub frame_id: u64,
    // pub ticks_count: u64,

    // Donchain Channel
    pub med_high: f64,
    pub med_low: f64,
    pub big_high: f64,
    pub big_low: f64,

    pub spreed_min: f64,
    pub spreed_max: f64,

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: SimpleCandle,
}

impl FrameMem {
    pub fn add_ticks(&mut self, ticks: TimeSerVec<Tick>) {
        if ticks.len() == 0 {
            println!(">> Trades are empty.");
            // return Err(TErr::EmptyTradesErr);
            return;
        }
        for t in ticks.get_vec() {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > self.spreed_max {
                self.spreed_max = spread;
            }
            if spread < self.spreed_min || self.spreed_min == 0. {
                self.spreed_min = spread;
            }
        }

        self.ohlc = SimpleCandle::new(ticks.get_vec());
    }

    pub fn to_csv(&self) -> (FrameMem, SimpleCandle) {
        (self.clone(), self.ohlc.clone())
    }
}

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
            open_time_str: "".to_string(),
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
