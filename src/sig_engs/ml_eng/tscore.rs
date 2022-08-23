use super::*;
use crate::bar::MultiBarRes;
use crate::ta::TDOut;
pub use serde::{Deserialize, Serialize};

// Trend Score
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TScore {
    pub bull: f64,
    pub bear: f64,
    pub diff: f64,
}

impl TScore {
    pub fn new(mbr: &MultiBarRes) -> TScore {
        let mut score = TScore::default();

        let mtd = &mbr.medium.big.ta.td;
        let btd = &mbr.major.big.ta.td;

        let t = btd.plus / btd.minus;
        if t > 1. {}
        let p1 = t.powi(2);
        // score.bull = p1;
        let big_s_bull = set_str(btd.plus / btd.minus);
        let big_s_bear = -set_str(btd.minus / btd.plus);

        let med_s_bull = set_str(mtd.plus / mtd.minus);
        let med_s_bear = -set_str(mtd.minus / mtd.plus);

        score.bull = (big_s_bull * 2. + med_s_bull * 1.) / 1.;
        score.bear = (big_s_bear * 2. + med_s_bear * 1.) / 1.;

        if big_s_bull < 0.1 {
            score.bull = 0.;
        }
        if big_s_bear.abs() < 0.1 {
            score.bear = 0.;
        }

        score.diff = score.bull + score.bear;

        score
    }
}
fn set_str(t: f64) -> f64 {
    // println!("t: {}", t);

    // let t = td.plus / td.minus;
    let r = if t.abs() > 1. {
        t.powi(2)
    } else {
        0.
        // t.powi(2)
    };
    let rt = if r < 0.8 {
        0.
    } else if r < 1.8 {
        1.
    } else if r < 3. {
        2.
    } else if r > 5. {
        3.
    } else {
        4.
    };

    rt
}
