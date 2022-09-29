use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;
// Impl 2 Meher 1401
// This indicator is based on MaMom and with added features like Vel

// Line Direction with Momentums

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineDirOut {
    pub ma: f64,
    // pub ma_sum: f64,
    // pub ma_count: i32,
    pub mom1: f64,
    pub mom2: f64,
    pub mom1_count: i32,
    // pub mom2_count: i32,
    pub mom1_sum: f64,
    // pub mom2_sum: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineDir {
    period: usize,
    ma: EMA,
    mom1: Momentum,
    mom2: Momentum,
    mom1_arr: VecDeque<f64>,
}

impl LineDir {
    // set period=0 to disable ma
    pub fn new(period: usize, mom1_diff: usize, mom2_diff: usize) -> TAResult<Self> {
        // not pass zero parameter to EMA
        let period_clean = if period == 0 { 1 } else { period };

        if mom1_diff == 0 || mom2_diff == 0 {
            Err(TAErr::WrongArgs)
        } else {
            Ok(Self {
                period,
                ma: EMA::new(period_clean)?,
                // window: Window::new(mom_diff + 1)?,
                mom1: Momentum::new(mom1_diff)?,
                mom2: Momentum::new(mom2_diff)?,
                mom1_arr: VecDeque::with_capacity(120),
            })
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> LineDirOut {
        let ma = if self.period == 0 {
            next_val
        } else {
            self.ma.next(next_val)
        };

        // let ma = self.ma.next(next);
        let mom1 = self.mom1.next(ma);
        let mom2 = self.mom2.next(mom1);

        // MA info
        self.mom1_arr.push_front(mom1);
        if self.mom1_arr.len() == 100 {
            self.mom1_arr.pop_back();
        }
        let (mom1_sum, mom1_count) = _get_ma_info(&self.mom1_arr, mom1);

        LineDirOut {
            ma,
            mom1,
            mom2,
            mom1_count,
            mom1_sum,
        }
    }
}

fn _get_ma_info(ma_mom_arr: &VecDeque<f64>, ma_mom: f64) -> (f64, i32) {
    let mut ma_sum_un = 0.;
    let mut count = 0;
    for m in ma_mom_arr.iter() {
        let m = *m;
        if ma_mom > 0. && m >= 0. {
            ma_sum_un += m;
            count += 1;
        } else if ma_mom < 0. && m <= 0. {
            ma_sum_un += m;
            count -= 1;
        } else {
            break;
        }
    }
    (ma_sum_un, count)
}

impl Default for LineDir {
    fn default() -> Self {
        Self::new(10, 5, 5).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(LineDir::new(0, 0, 0).is_err());
        assert!(LineDir::new(1, 1, 0).is_ok());
    }

    #[test]
    fn test_next() {}

    #[test]
    fn test_default() {
        LineDir::default();
    }
}
