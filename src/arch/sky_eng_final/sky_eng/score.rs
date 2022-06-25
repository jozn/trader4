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
        let mut score = Score::default();
        score.set_macd(f);
        // score.set_macd_big(f);
        // score.set_dmi(f);
        score.set_trend(f);

        score.diff = score.bull - score.bear;

        score
    }

    pub fn set_macd(&mut self, f: &SFrame) {
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        // Bull
        if pta.macd.histogram > 0. {
            // blue line above orange line (macd and signal line)
            self.bull += 1;
        }
        if pta.macd.color == 2. || pta.macd.color == -1. {
            self.bull += 1;
        }

        // Bear
        if pta.macd.histogram < 0. {
            self.bear += 1;
        }
        if pta.macd.color == -2. || pta.macd.color == 1. {
            self.bear += 1;
        }
    }
    pub fn set_macd_big(&mut self, f: &SFrame) {
        let bta = &f.bar_medium.big.ta;

        // Bull
        if bta.macd.histogram > 0. {
            // blue line above orange line (macd and signal line)
            self.bull += 1;
        }
        if bta.macd.color == 2. || bta.macd.color == -1. {
            self.bull += 1;
        }

        // Bear
        if bta.macd.histogram < 0. {
            self.bear += 1;
        }
        if bta.macd.color == -2. || bta.macd.color == 1. {
            self.bear += 1;
        }
    }

    // DMI +5
    pub fn set_dmi(&mut self, f: &SFrame) {
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        // Bull
        if pta.dmi.plus > pta.dmi.minus {
            self.bull += 2;
            if pta.dmi.adx > pta.dmi.plus {
                self.bull += 2;
            }
        }

        // Bear
        if pta.dmi.plus < pta.dmi.minus {
            self.bear += 2;
            if pta.dmi.adx > pta.dmi.minus {
                self.bear += 2;
            }
        }
    }

    // +3 - use big time frame
    pub fn set_trend(&mut self, f: &SFrame) {
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        // trend big
        if bta.trend.is_bullish() {
            self.bull += 2;
            if pta.trend.is_bullish() {
                self.bull += 1;
            }
        }

        if bta.trend.is_bearish() {
            self.bear += 2;
            if pta.trend.is_bearish() {
                self.bear += 1;
            }
        }
    }

    pub fn new_bk(f: &SFrame) -> Score {
        let mut bull = 0;
        let mut bear = 0;
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

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
