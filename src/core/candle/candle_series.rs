use super::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandleSeries {
    pub ticks: TickVec<Tick>,
    pub small: KlineHolderFrame,
    pub medium: KlineHolderFrame,
    pub big: KlineHolderFrame,
    // pub small_tick_count: f64,
    pub small_tick_count: u64,
}

// Holder of small, medium, big, ... Frame.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KlineHolderFrame {
    pub small_multi: u64, // how many of small candle this candle should be
    pub klines: KlineSerVec<Kline>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandleConfig {
    pub small_tick: u64,
    pub medium_tick: u64,
    pub big_tick: u64,
    pub vel1_period: u64,
    pub vel2_period: u64,
}

pub type CandleSeriesDiff = CandleSeries; // Only trades is not set

impl CandleSeries {
    pub fn new(cfg: &CandleConfig) -> Self {
        Self {
            ticks: TickVec::new(),
            small: Default::default(),
            medium: KlineHolderFrame {
                small_multi: cfg.medium_tick,
                klines: Default::default(),
            },
            big: KlineHolderFrame {
                small_multi: cfg.big_tick,
                klines: Default::default(),
            },
            small_tick_count: cfg.small_tick,
        }
    }

    pub fn add_ticks(&mut self, ticks: TimeSerVec<Tick>) -> TResult<CandleSeriesDiff> {
        if ticks.len() == 0 {
            println!(">> Trades are empty.");
            return Err(TErr::EmptyTradesErr);
        }

        let first = ticks.first().unwrap();
        let _last = ticks.last().unwrap();

        // New trades times must be newer than the last one.
        match self.ticks.last() {
            None => {}
            Some(last) => {
                let new_first = first;
                if !(new_first.time_s >= last.data.time_s) {
                    println!(">> Ticks are invalid (they must be newer).");
                    return Err(TErr::TickTimeErr);
                }
            }
        }

        let mut first_bucket = 0;
        for (idx, t) in ticks.get_vec().iter().enumerate() {
            if idx == 0 {
                let res = self.ticks.push(t.clone()).unwrap(); // todo unwerp
                first_bucket = res.id;
            } else {
                self.ticks.push(t.clone()).unwrap(); // todo fix unearp
            }
        }

        self.add_new_small_klines_from_new_ticks();
        self.add_other_klines();

        self.get_diff_klines(first_bucket)
    }

    fn add_new_small_klines_from_new_ticks(&mut self) {
        let last_kline = self.small.klines.last();
        let mut last_bucket = match last_kline {
            None => 0,
            Some(kline) => {
                // _kline_bucket_id = kline.bucket;
                kline.bucket
            }
        };
        let tip_ticks = self.ticks.get_from(last_bucket);

        let mut stage_ticks = vec![];
        let mut count = 0;

        for trade_item in tip_ticks.iter() {
            let trade = trade_item.data.as_ref();
            stage_ticks.push(trade_item.clone());
            count += 1;

            if count >= self.small_tick_count {
                let mut kline = aggregate_tickss_to_kline(&TickVec::transform_seq(&stage_ticks));
                let first_tick = stage_ticks.first().unwrap();
                kline.bucket = first_tick.id;
                self.small.klines.push_replace(kline);
                count = 0;
                stage_ticks.clear();
            }
        }

        // Check for remaing not fully formed tikcs - Not fully formed candle tip
        if stage_ticks.len() > 0 {
            // copy of above code
            let mut kline = aggregate_tickss_to_kline(&TickVec::transform_seq(&stage_ticks));
            let first_tick = stage_ticks.first().unwrap();
            kline.bucket = first_tick.id;
            self.small.klines.push_replace(kline);
        }
    }

    fn get_diff_klines(&self, first_bucket: u64) -> TResult<CandleSeriesDiff> {
        let res = CandleSeriesDiff {
            ticks: Default::default(),
            small: KlineHolderFrame {
                small_multi: 0,
                klines: self.small.klines.get_from_lower(first_bucket),
            },
            medium: KlineHolderFrame {
                small_multi: self.medium.small_multi,
                klines: self.medium.klines.get_from_lower(first_bucket),
            },
            big: KlineHolderFrame {
                small_multi: self.big.small_multi,
                klines: self.big.klines.get_from_lower(first_bucket),
            },
            small_tick_count: 0, // not relevent
        };

        Ok(res)
    }

    fn add_other_klines(&mut self) {
        regenerate_last_other_klines(&self.small.klines, &mut self.medium, self.small_tick_count);
        regenerate_last_other_klines(&self.small.klines, &mut self.big, self.small_tick_count);
    }
}

impl Default for CandleConfig {
    fn default() -> Self {
        Self {
            small_tick: SMALL_TICK,
            medium_tick: MEDIUM_TICK,
            big_tick: BIG_TICK,
            vel1_period: 50,
            vel2_period: 100,
        }
    }
}

fn aggregate_tickss_to_kline(ticks: &Vec<Tick>) -> Kline {
    let num = ticks.len() as u32;
    assert!(num > 0);
    let bucket_id = 0;

    let first = ticks.first().unwrap().clone();
    let last = ticks.last().unwrap().clone();

    let mut high = 0.;
    let mut low = f64::MAX;
    let mut volume = 0.;

    for trade in ticks.iter() {
        if trade.price_raw > high {
            high = trade.price_raw;
        }

        if trade.price_raw < low {
            low = trade.price_raw;
        }

        volume += trade.qty;
    }

    let kline = Kline {
        kid: 0,
        open_time: first.time_s,
        close_time: last.time_s,
        bucket: bucket_id, // this should be override in codes who calls this
        tick_count: num,
        kline_num: -1, // -1 shows kline is build from ticks - just in samll
        open: first.price_raw,
        high: high,
        low: low,
        close: last.price_raw,
        volume: volume,
    };
    kline
}

fn regenerate_last_other_klines(
    small_klines: &KlineSerVec<Kline>,
    candle_raw: &mut KlineHolderFrame,
    small_volume: u64,
) {
    let total_volume = candle_raw.small_multi * small_volume;

    let last = candle_raw.klines.last();
    let mut from_bucket = match last {
        None => {
            // add first
            0
        }
        Some(k) => {
            //rebuild
            k.bucket
        }
    };

    let num_of_klines = candle_raw.small_multi as u64;
    loop {
        let arr = small_klines.get_from_limit(from_bucket, num_of_klines);
        if arr.len() == 0 {
            break;
        }

        let last = arr.last().unwrap().clone();

        let kline = sum_klines_from_small(arr);
        kline.validate();

        candle_raw.klines.push_replace(kline);

        // For the last kline if we do not break we go into infinite loop > as get_from_limit will return last kline
        if from_bucket == last.bucket {
            break;
        }

        from_bucket = last.bucket + 1;
    }
}

fn sum_klines_from_small(arr: KlineSerVec<Kline>) -> Kline {
    assert!(arr.len() > 0);
    // println!(">>>>>>>>>****: {}  ", arr.len());

    let first = arr.first().unwrap().clone();
    let last = arr.last().unwrap().clone();

    let mut high = 0.;
    let mut low = f64::MAX;
    let mut volume = 0.;
    let mut ticks_count = 0;

    for kline in arr.iter() {
        if kline.high > high {
            high = kline.high;
        }

        if kline.low < low {
            low = kline.low;
        }

        volume += kline.volume;
        ticks_count += kline.tick_count;
    }

    Kline {
        kid: 0,
        open_time: first.open_time,
        close_time: last.close_time,
        bucket: first.bucket, // todo
        tick_count: ticks_count,
        kline_num: arr.len() as i32,
        open: first.open,
        high: high,
        low: low,
        close: last.close,
        volume: volume,
    }
}
