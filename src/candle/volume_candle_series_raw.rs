use super::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeCandleSeriesRaw {
    // pub trades: SerVec<CSVTradeRecord>,
    pub trades: TickVec<Tick>,
    pub small: VolumeCandleTimeFrameRaw,
    pub medium: VolumeCandleTimeFrameRaw,
    pub big: VolumeCandleTimeFrameRaw,
    pub small_volume_size: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct VolumeCandleTimeFrameRaw {
    pub small_multi: u64, // how many of small candle this candle should be
    pub klines: KlineSerVec<Kline>,
}

pub type VolumeVolCandleAddDiff = VolumeCandleSeriesRaw; // Only trades is not set

impl VolumeCandleSeriesRaw {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_trades(&mut self, trades: SerVec<Tick>) -> TResult<VolumeVolCandleAddDiff> {
        if trades.len() == 0 {
            println!(">> Trades are empty.");
            return Err(TErr::EmptyTradesErr);
        }

        let first = trades.first().unwrap();
        let _last = trades.last().unwrap();

        // New trades times must be newer than the last one.
        match self.trades.last() {
            None => {}
            Some(last) => {
                let new_first = first;
                if !(new_first.time >= last.data.time) {
                    println!(">> Trades are invalid (they must be newer).");
                    return Err(TErr::TradesTimeErr);
                }
            }
        }

        let mut first_bucket = 0;
        for (idx, t) in trades.get_vec().iter().enumerate() {
            if idx == 0 {
                let res = self.trades.push(t.clone()).unwrap(); // todo unwerp
                first_bucket = res.id;
            } else {
                self.trades.push(t.clone()).unwrap(); // todo fix unearp
            }
        }

        self.add_new_small_klines_from_new_trades();
        self.add_other_klines();

        //todo return VolCandleAddDiff
        // Err(TErr::TradesTimeErr)
        // Ok(())
        self.get_diff_klines(first_bucket)
    }

    fn add_new_small_klines_from_new_trades(&mut self) {
        // let mut _kline_bucket_id = 0;
        let last_kline = self.small.klines.last();
        let mut last_bucket = match last_kline {
            None => 0,
            Some(kline) => {
                // _kline_bucket_id = kline.bucket;
                kline.bucket
            }
        };
        // println!("new trades {} 111111111111", last_bucket,);
        let tip_trades = self.trades.get_from(last_bucket);

        // println!("new trades {} : {:#?}", last_bucket, &tip_trades.len());
        let mut stage_trades = vec![];
        let mut sum = 0.0;

        for trade_item in tip_trades.iter() {
            let trade = trade_item.data.as_ref();
            stage_trades.push(trade_item.clone());
            sum += trade.qty;

            if sum > self.small_volume_size {
                let mut kline = aggregate_trades_to_kline(&TickVec::transform_seq(&stage_trades));
                let first_trade = stage_trades.first().unwrap();
                kline.bucket = first_trade.id;
                self.small.klines.push_replace(kline);
                sum = 0.0;
                stage_trades.clear();
            }
        }

        // Not fully formed candle tip
        if stage_trades.len() > 0 {
            // copy of above code
            let mut kline = aggregate_trades_to_kline(&TickVec::transform_seq(&stage_trades));
            let first_trade = stage_trades.first().unwrap();
            kline.bucket = first_trade.id;
            self.small.klines.push_replace(kline);
        }
    }

    fn get_diff_klines(&self, first_bucket: u64) -> TResult<VolumeVolCandleAddDiff> {
        let res = VolumeVolCandleAddDiff {
            trades: Default::default(),
            small: VolumeCandleTimeFrameRaw {
                small_multi: 0,
                klines: self.small.klines.get_from_lower(first_bucket),
            },
            medium: VolumeCandleTimeFrameRaw {
                small_multi: self.medium.small_multi,
                klines: self.medium.klines.get_from_lower(first_bucket),
            },
            big: VolumeCandleTimeFrameRaw {
                small_multi: self.big.small_multi,
                klines: self.big.klines.get_from_lower(first_bucket),
            },
            small_volume_size: 0.0, // not relevent
        };

        Ok(res)
    }

    fn add_other_klines(&mut self) {
        regenerate_last_other_klines(&self.small.klines, &mut self.medium, self.small_volume_size);
        regenerate_last_other_klines(&self.small.klines, &mut self.big, self.small_volume_size);
    }
}

impl Default for VolumeCandleSeriesRaw {
    fn default() -> Self {
        Self {
            trades: TickVec::new(),
            small: Default::default(),
            medium: VolumeCandleTimeFrameRaw {
                small_multi: MEDIUM_VOLUME,
                klines: Default::default(),
            },
            big: VolumeCandleTimeFrameRaw {
                small_multi: BIG_VOLUME,
                klines: Default::default(),
            },
            small_volume_size: SMALL_VOLUME,
        }
    }
}

fn aggregate_trades_to_kline(trades: &Vec<Tick>) -> Kline {
    let num = trades.len() as u32;
    assert!(num > 0);
    let bucket_id = 7;

    let first = trades.first().unwrap().clone();
    let last = trades.last().unwrap().clone();

    let mut high = 0.;
    let mut low = f64::MAX;
    let mut volume = 0.;

    for trade in trades.iter() {
        if trade.price > high {
            high = trade.price;
        }

        if trade.price < low {
            low = trade.price;
        }

        volume += trade.qty;
    }

    // println!(">>>> Volume #{:#?}", volume);

    let kline = Kline {
        open_time: first.time,
        close_time: last.time,
        bucket: bucket_id,
        // num: num,
        // trades: arr,
        // kline_num: -9,
        // trades: SerVec::new(),
        tick_count: num,
        kline_num: -1,
        open: first.price,
        high: high,
        low: low,
        close: last.price,
        volume: volume,
    };
    kline
}

fn regenerate_last_other_klines(
    small_klines: &KlineSerVec<Kline>,
    candle_raw: &mut VolumeCandleTimeFrameRaw,
    small_volume: f64,
) {
    let last = candle_raw.klines.last();
    let total_volume = candle_raw.small_multi as f64 * small_volume;

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
        // println!("{}", arr.len());

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
    let mut trades_num = 0;

    for kline in arr.iter() {
        if kline.high > high {
            high = kline.high;
        }

        if kline.low < low {
            low = kline.low;
        }

        volume += kline.volume;
        trades_num += kline.tick_count;

        //println!(">> Volume {}  ", volume);
    }

    Kline {
        open_time: first.open_time,
        close_time: last.close_time,
        bucket: first.bucket, // todo
        // num: trades_num,
        // kline_num: arr.len() as i32,
        // trades: arr,
        // trades: SerVec::new(),
        tick_count: trades_num,
        kline_num: arr.len() as i32,
        open: first.open,
        high: high,
        low: low,
        close: last.close,
        volume: volume,
    }
}
