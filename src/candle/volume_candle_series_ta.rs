use super::*;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VolumeCandleSeriesTA {
    pub klines: VolumeCandleSeriesRaw,
    pub small: VolumeCandleTimeFrameTA,
    pub medium: VolumeCandleTimeFrameTA,
    pub big: VolumeCandleTimeFrameTA,

    // Ticking is just for offline data analyse not any for realtime
    pub ticking_tip: VolumeTickingTipHolder,
    pub ticking: VolSerVec<KlineTATick>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VolumeCandleTimeFrameTA {
    pub length_ms: u64,
    pub ta_holder: TAMethods,
    pub klines_ta: VolSerVec<KlineTA>,
    pub kline_ta_tip: Option<KlineTA>,
}

impl VolumeCandleSeriesTA {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_trades(&mut self, trades: SerVec<CSVTradeRecord>) {
        let diff = self.klines.add_trades(trades);
        self.process_diff(diff);
    }

    fn process_diff(&mut self, diff: TResult<VolumeVolCandleAddDiff>) {
        if diff.is_err() {
            return;
        }
        let diff_cp = diff.clone();
        let diff = diff.unwrap();

        fn doer(ctf: VolumeCandleTimeFrameRaw, cta: &mut VolumeCandleTimeFrameTA) {
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

    fn process_tip(&mut self, diff: TResult<VolumeVolCandleAddDiff>) {
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
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KlineTATick {
    pub small: KlineTA,
    pub medium: KlineTA,
    pub big: KlineTA,
}

impl VolumeId for KlineTATick {
    fn get_volume_id(&self) -> u64 {
        self.small.kline.bucket
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VolumeTickingTipHolder {
    pub small: VolumeTickingTipEntry,
    pub medium: VolumeTickingTipEntry,
    pub big: VolumeTickingTipEntry,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VolumeTickingTipEntry {
    pub method: TAMethods,
    pub kline: Kline,
}
