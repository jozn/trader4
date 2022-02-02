use super::*;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Score {
    pub bull: i32,
    pub bear: i32,
    pub diff: i32,
}

impl Score {
    pub fn new(f: &SFrame) -> Score {
        let mut bull = 0;
        let mut bear = 0;
        let pta = &f.bar.primary.ta;
        let bta = &f.bar.big.ta;

        if pta.macd.histogram > 0. {
            bull += 1;
        }
        // if f.roc_macd > 0. {
        if pta.macd.dir > 0. {
            bull += 1;
        }

        if pta.macd.histogram < 0. {
            bear += 1;
        }
        // if f.roc_macd < 0. {
        if pta.macd.dir < 0. {
            bear += 1;
        }

        // DMI +5
        if pta.dmi.plus > pta.dmi.minus {
            bull += 2;
            if pta.dmi.adx > pta.dmi.plus {
                bull += 2;
            }
        }

        // DMI - bear +5
        if pta.dmi.plus < pta.dmi.minus {
            bear += 2;
            if pta.dmi.adx > pta.dmi.minus {
                bear += 2;
            }
        }

        // trend big
        if bta.trend.is_bullish() {
            bull += 3;
        }

        if bta.trend.is_bearish() {
            bear += 3;
        }

        let diff = bull - bear;

        Score { bull, bear, diff }
    }
}
