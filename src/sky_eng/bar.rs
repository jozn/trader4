use prost::Message;
use serde::{Deserialize, Serialize};

use super::*;
use crate::base::OHLCV;
use crate::collector::row_data::BTickData;
use crate::ta;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bar {
    pub seq: i32,
    pub open_time: i64, // in mill seconds
    pub close_time: i64,
    pub bucket: i64,
    pub tick_count: u32,
    pub kline_num: i32, // -1: from trades sums >0 sums of klines
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(skip)]
    pub ta: BarTA,
}

impl Bar {
    pub fn new(ticks: &Vec<BTickData>) -> Bar {
        let counts = ticks.len() as u32;
        assert!(counts > 0);
        let bucket_id = 0;

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

        let bar = Bar {
            seq: 0,
            open_time: first.timestamp,
            close_time: last.timestamp,
            bucket: bucket_id, // this should be override in codes who calls this
            tick_count: counts,
            kline_num: -1, // -1 shows kline is build from ticks - just in samll
            open: first.get_price(),
            high: high,
            low: low,
            close: last.get_price(),
            volume: volume,
            ta: Default::default(),
        };
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BarTA {
    pub atr: f64,
    pub macd: ta::MACDOutput,
    pub dmi: ta::DMIOutput,
    pub stoch: ta::StochRes,
    pub trend: ta::MATrendOut,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BarConfig {
    pub primary_ticks: u64,
    pub big_ticks: u64, // big must be multiple of primary
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimaryHolder {
    pub primary: Bar,
    pub big: Bar,
    pub finish_primary: bool,
    pub finish_big: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BarSeries {
    pub cfg: BarConfig,
    pub primary_seq: i32,
    pub big_seq: i32,
    pub ticks_primary: Vec<BTickData>,
    pub ticks_big: Vec<BTickData>,
    pub bars_primary: Vec<PrimaryHolder>,
    pub bars_big: Vec<Bar>,
    primary_ta: TAMethods,
    big_ta: TAMethods,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAMethods {
    pub atr: ta::ATR,
    pub macd: ta::MACD,
    pub dmi: ta::DMI,
    pub stoch: ta::Stoch,
    pub trend: ta::MATrend,
}

impl TAMethods {
    pub fn new(cfg: &BarConfig) -> Self {
        Self {
            atr: ta::ATR::new(14).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            dmi: ta::DMI::new(14, 14).unwrap(),
            stoch: ta::Stoch::new(14, 3, 5).unwrap(),
            trend: ta::MATrend::new(10).unwrap(),
        }
    }
}

impl BarSeries {
    pub fn new(cfg: &BarConfig) -> BarSeries {
        assert!(cfg.big_ticks > cfg.primary_ticks);
        assert!(cfg.big_ticks % cfg.primary_ticks == 0);

        BarSeries {
            cfg: cfg.clone(),
            primary_seq: 0,
            big_seq: 0,
            ticks_primary: vec![],
            ticks_big: vec![],
            bars_primary: vec![],
            bars_big: vec![],
            primary_ta: TAMethods::new(&cfg),
            big_ta: TAMethods::new(&cfg),
        }
    }

    pub fn add_ticks(&mut self, ticks: Vec<BTickData>) {
        if ticks.len() == 0 {
            println!(">> Trades are empty.");
            return;
        }

        let mut last_time = ticks.first().unwrap().timestamp;
        for t in &ticks {
            if t.timestamp < last_time {
                println!(">> Ticks time are invalid");
                debug_assert!(t.timestamp < last_time);
                return; // in live
            }
            last_time = t.timestamp;
        }

        for t in &ticks {
            self.add_tick_mut(t);
        }
    }

    pub fn add_tick_mut(&mut self, tick: &BTickData) -> Option<PrimaryHolder> {
        self.ticks_primary.push(tick.clone());
        self.ticks_big.push(tick.clone());

        if self.ticks_primary.len() == self.cfg.primary_ticks as usize {
            let mut finish_big = false;
            self.primary_seq += 1;
            let mut bar_prim = Bar::new(&self.ticks_primary);
            bar_prim.seq = self.primary_seq;
            bar_prim.ta = cal_indicators(&mut self.primary_ta, &bar_prim);

            let mut bar_big = Bar::new(&self.ticks_big);
            bar_big.seq = self.big_seq;
            bar_big.ta = cal_indicators(&mut self.primary_ta, &bar_big);

            if self.ticks_big.len() == self.cfg.big_ticks as usize {
                self.big_seq += 1;
                bar_big.seq = self.big_seq;
                self.ticks_big.clear();
                self.bars_big.push(bar_big.clone());
                finish_big = true;
            }

            self.ticks_primary.clear();
            let ph = PrimaryHolder {
                primary: bar_prim.clone(),
                big: bar_big.clone(),
                finish_primary: true,
                finish_big,
            };
            self.bars_primary.push(ph.clone());

            Some(ph)
        } else {
            // in here we could also build new Bars without changing states
            None
        }
    }
}

pub fn cal_indicators(tam: &mut TAMethods, bar: &Bar) -> BarTA {
    let _price = bar.hlc3();
    BarTA {
        atr: tam.atr.next(&bar),
        macd: tam.macd.next(bar.close),
        dmi: tam.dmi.next(&bar),
        stoch: tam.stoch.next(&bar),
        trend: tam.trend.next(&bar),
    }
}