use super::*;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CandleSeriesTA {
    pub klines: CandleSeries,
    pub small: KlineHolderFrameTA,
    pub medium: KlineHolderFrameTA,
    pub big: KlineHolderFrameTA,

    // todo add flag is we should process ticking
    // Ticking is just for offline data analyse not any for realtime
    pub ticking_tip: TickingTipHolderTA,
    pub ticking: KlineSerVec<KlineTATick>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct KlineHolderFrameTA {
    pub length_ms: u64,
    pub ta_holder: TAMethods,
    pub klines_ta: KlineSerVec<KlineTA>,
    pub kline_ta_tip: Option<KlineTA>,
}

impl CandleSeriesTA {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_trades(&mut self, trades: TimeSerVec<Tick>) {
        let diff = self.klines.add_ticks(trades);
        self.process_diff(diff);
    }

    fn process_diff(&mut self, diff: TResult<CandleSeriesDiff>) {
        if diff.is_err() {
            return;
        }
        let diff_cp = diff.clone();
        let diff = diff.unwrap();

        fn doer(ctf: KlineHolderFrame, cta: &mut KlineHolderFrameTA) {
            for k in ctf.klines.get_vec() {
                let kta = cal_indicators(&mut cta.ta_holder.clone(), k);
                // cta.klines_ta.push_replace(kta);
                match cta.kline_ta_tip.clone() {
                    None => cta.kline_ta_tip = Some(kta),
                    Some(per_kline_ta) => {
                        if per_kline_ta.kline.bucket == kta.kline.bucket {
                            cta.kline_ta_tip = Some(kta)
                        } else {
                            let kta_per_final =
                                cal_indicators(&mut cta.ta_holder, &per_kline_ta.kline);
                            cta.klines_ta.push_replace(kta_per_final);
                            cta.kline_ta_tip = Some(kta)
                        }
                    }
                }
            }
        }

        doer(diff.small, &mut self.small);
        doer(diff.medium, &mut self.medium);
        doer(diff.big, &mut self.big);

        self.process_tip(diff_cp);
    }

    fn process_tip(&mut self, diff: TResult<CandleSeriesDiff>) {
        if diff.is_err() {
            return;
        }
        let diff = diff.unwrap();

        for new_small_kline in diff.small.klines.get_vec() {
            let bucket = new_small_kline.bucket;

            let new_medium_kline = diff.medium.klines.get_single(bucket).unwrap();
            let new_big_kline = diff.big.klines.get_single(bucket).unwrap();

            if self.ticking_tip.small.kline.bucket == new_small_kline.bucket {
                // Nothing - only when a Small bucket is fulled process in the next code block
            } else {
                // Small bucket is now filled processed

                // Small
                let kline_s = self.ticking_tip.small.kline.clone();
                let kta_s = cal_indicators(&mut self.ticking_tip.small.method, &kline_s);

                // Medium
                let kline_m = self.ticking_tip.medium.kline.clone();
                let kta_m = if kline_m.bucket == new_medium_kline.bucket {
                    cal_indicators(&mut self.ticking_tip.medium.method.clone(), &kline_m)
                // Clone
                } else {
                    cal_indicators(&mut self.ticking_tip.medium.method, &kline_m)
                    // Modify
                };

                // Big
                let kline_b = self.ticking_tip.big.kline.clone();
                let kta_b = if kline_b.bucket == new_big_kline.bucket {
                    cal_indicators(&mut self.ticking_tip.big.method.clone(), &kline_b)
                // Clone
                } else {
                    cal_indicators(&mut self.ticking_tip.big.method, &kline_b) // Modify
                };

                let kline_tick = KlineTATick {
                    small: kta_s,
                    medium: kta_m,
                    big: kta_b,
                };

                self.ticking.push_replace(kline_tick);
            }

            // Set tips to new klines
            self.ticking_tip.small.kline = new_small_kline.clone();
            self.ticking_tip.medium.kline = new_medium_kline;
            self.ticking_tip.big.kline = new_big_kline;
        }
    }

    pub fn print_klines(&self) {
        println!("SMALL: {:#?}", self.small);
        println!("MEDIUM: {:#?}", self.medium);
        println!("BIG: {:#?}", self.big);
    }

    pub fn print_ticking(&self) {
        println!("{:#?}", self.ticking);
        println!("{:#?}", self.ticking_tip);
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KlineTATick {
    pub small: KlineTA,
    pub medium: KlineTA,
    pub big: KlineTA,
}

impl KlineId for KlineTATick {
    fn get_kline_id(&self) -> u64 {
        self.small.kline.bucket
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TickingTipHolderTA {
    pub small: TickingTipEntry,
    pub medium: TickingTipEntry,
    pub big: TickingTipEntry,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TickingTipEntry {
    pub method: TAMethods,
    pub kline: Kline,
}
