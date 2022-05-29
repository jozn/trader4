use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;

// This is a merge of Moving Average Momentum (ma_mom) and Vel indicators
//  Moving Average Momentum

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelMom {
    period: usize,
    // ma: EMA,
    ma: WMA,
    last_ma: f64,
    window_ma: Window,
    ma_mom_arr: VecDeque<f64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VelMomRes {
    pub ma: f64,
    pub ma_mom: f64, // Moving Average Momentum
    pub ma_sum: f64, // comultive of direction of mv
    pub count: i32,  // all positive/negative candles counts
}

impl VelMom {
    pub fn new(period: usize, mom_diff: usize) -> TAResult<Self> {
        if period == 0 || mom_diff == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                period,
                // ma: EMA::new(period)?,
                ma: WMA::new(period)?,
                last_ma: 0.,
                window_ma: Window::new(mom_diff + 1)?,
                ma_mom_arr: VecDeque::with_capacity(1000),
            })
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> VelMomRes {
        let ma = self.ma.next(next_val);
        let _last = self.window_ma.push(ma);
        let tail = self.window_ma.tail().unwrap();

        let ma_mom = (ma - tail) * 10_000.;
        self.ma_mom_arr.push_front(ma_mom);
        if self.ma_mom_arr.len() == 1000 {
            self.ma_mom_arr.pop_back();
        }

        //todo: a bug for == 0.
        let mut ma_sum_un = 0.;
        let mut count = 0;
        for m in self.ma_mom_arr.iter() {
            let m = *m;
            if (ma_mom > 0. && m >= 0.) || (ma_mom < 0. && m <= 0.) {
                // if (ma_mom > 0. && m >= 0.) || (self.last_ma >= 0.  ) {
                ma_sum_un += m;
                count += 1;
            // } else if (ma_mom < 0. && m <= 0.) || ( self.last_ma <= 0. ) {
            //     ma_sum += m;
            //     count -= 1;
            } else {
                break;
            }
        }
        let ma_sum = if count == 0 {
            // println!("=== zero {:?}=====================",self.ma_mom_arr.len());
            let ma_sum_past = self.last_ma;
            self.last_ma = ma_sum_un;
            ma_sum_past
        } else {
            ma_sum_un
        };
        // self.last_ma = ma_sum_un;

        VelMomRes {
            ma,
            ma_mom,
            ma_sum,
            count,
        }
    }
}

impl Default for VelMom {
    fn default() -> Self {
        Self::new(9, 3).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(VelMom::new(0, 0).is_err());
        assert!(VelMom::new(1, 1).is_ok());
    }

    #[test]
    fn test_next() {}

    #[test]
    fn test_default() {
        VelMom::default();
    }
}
