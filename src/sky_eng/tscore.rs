use super::*;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TScore {
    pub bull: i32,
    pub bear: i32,
    pub diff: i32,
}

impl TScore {
    pub fn new(f: &SFrame) -> TScore {
        let mut score = TScore::default();
        // score.set_macd(f);
        // score.set_macd_big(f);
        score.set_dmi(f);
        // score.set_trend(f);

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

    pub fn set_dmi(&mut self, f: &SFrame) {
        let mbta = &f.bar_major.big.ta;
        let mpta = &f.bar_major.primary.ta;
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        if true {
            // add both green and red independetly
            self.bull += cal_dmi_score(mbta.dmi.plus);
            self.bear += cal_dmi_score(mbta.dmi.minus);
        } else {
        }
    }

    // +3 - use big time frame
    pub fn set_trend(&mut self, f: &SFrame) {
        let mbta = &f.bar_major.big.ta;
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        // trend big
        if mbta.trend.is_bullish() {
            self.bull += 2;
            if bta.trend.is_bullish() {
                self.bull += 2;
                if pta.trend.is_bullish() {
                    self.bull += 1;
                }
            }
        }

        if mbta.trend.is_bearish() {
            self.bear += 2;
            if bta.trend.is_bearish() {
                self.bear += 2;
                if pta.trend.is_bearish() {
                    self.bear += 1;
                }
            }
        }
    }

    // DMI +5
    pub fn set_dmi_dep(&mut self, f: &SFrame) {
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
}

fn cal_dmi_score(num: f64) -> i32 {
    let d = (num - 10.).max(0.).round() as u32;
    let s = d / 5; // from 15 to 35 - step 5
    let s = s.min(5);
    s as i32
}
