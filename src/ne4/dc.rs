use super::*;
use crate::candle::Tick;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NEDC {
    pub perc: f64,    // (price -low) / (high - low) of dc
    pub h_high: bool, // higher high
    pub l_low: bool,  // lower low
    pub hh: f64,      // 1 up _ 0 neturalze _ -1 down
    pub ll: f64,      // 1 up _ 0 neturalze _ -1 down
    pub dis_bull: u64,
    pub dis_bear: u64,
    pub buy: bool,
    pub sell: bool,
}

impl NEDC {
    pub fn new(frame: &NEFrame, tick: &Tick, rel: &FramesRels) -> Self {
        let perc = (tick.price_raw - frame.big_low) / (frame.big_high - frame.big_low);

        let pre = rel.frames2.last().unwrap();
        let last = frame;

        let mut dc = NEDC {
            perc: perc,
            h_high: false,
            l_low: false,
            hh: 0.0,
            ll: 0.0,
            dis_bull: 0,
            dis_bear: 0,
            buy: false,
            sell: false,
        };

        // going up?
        if last.big_high > pre.big_high {
            dc.h_high = true;
            if last.trd_ad < 0. {
                dc.sell = true;
            }
        }

        if last.big_low < pre.big_low {
            dc.l_low = true;
            if last.trd_ad > 0. {
                dc.buy = true;
            }
        }

        if dc.h_high == true && dc.l_low == true {
            dc.h_high = false;
            dc.l_low = false;
        }
        dc
    }
}
