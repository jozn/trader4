use super::*;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TScore {
    pub bull: f64,
    pub bear: f64,
    pub diff: f64,
}

impl TScore {
    pub fn new(f: &SFrame) -> TScore {
        let mut score = TScore::default();
        // score.set_macd(f);
        // score.set_macd_big(f);
        // score.set_dmi(f);
        score.set_trend(f);

        score.diff = score.bull - score.bear;

        score
    }
    // +2.5
    pub fn set_dmi(&mut self, f: &SFrame) {
        let mbta = &f.bar_major.big.ta;
        let mpta = &f.bar_major.primary.ta;
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        if false {
            // add both green and red independetly
            self.bull += cal_dmi_score(mbta.dmi.plus);
            self.bear += cal_dmi_score(mbta.dmi.minus);
        } else {
            let up_major = cal_dmi_score(mbta.dmi.plus);
            let up_med = cal_dmi_score(mpta.dmi.plus);
            let down_major = cal_dmi_score(mbta.dmi.minus);
            let down_med = cal_dmi_score(mpta.dmi.minus);

            let diff = up_major + up_med - down_major - down_med;
            let diff = diff / 4.;
            if false {
                self.bull += up_major + up_med;
                self.bear += down_major + down_med;
            } else {
                if diff > 0. {
                    self.bull += diff;
                } else {
                    self.bear += diff.abs();
                }
            }
        }
    }

    // +3 - use big time frame
    pub fn set_trend(&mut self, f: &SFrame) {
        let mbta = &f.bar_major.big.ta;
        let pta = &f.bar_medium.primary.ta;
        let bta = &f.bar_medium.big.ta;

        // trend big
        if mbta.trend.is_bullish() {
            self.bull += 2.;
            if bta.trend.is_bullish() {
                self.bull += 2.;
                if pta.trend.is_bullish() {
                    self.bull += 1.;
                }
            }
        }

        if mbta.trend.is_bearish() {
            self.bear += 2.;
            if bta.trend.is_bearish() {
                self.bear += 2.;
                if pta.trend.is_bearish() {
                    self.bear += 1.;
                }
            }
        }
    }
}

fn cal_dmi_score(num: f64) -> f64 {
    let d = (num - 10.).max(0.).round() as u32;
    let s = d / 5; // from 15 to 35 - step 5
    let s = s.min(5);
    s as f64
}
