use core::panicking::panic;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;
// Impl Date: Mehr 1401

// VelTrend is new ideas of Vel and VelMom with the goal of trend direction with momentum
//  of momentum of moving averges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelTrend {
    // ma: EMA,
    ma: WMA,
    last_ma: f64,
    mom1: Momentum,
    mom2: Momentum,
    ma_mom_arr: VecDeque<f64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VelMomRes {
    pub ma: f64,
    pub ma_mom: f64, // Moving Average Momentum
    pub ma_sum: f64, // comultive of direction of mv
    pub count: i32,  // all positive/negative candles counts
}

impl VelTrend {
    pub fn new(period: usize, mom_diff: usize) -> TAResult<Self> {
        if period == 0 || mom_diff == 0 {
            Err(TAErr::WrongArgs)
        } else {
            // Ok(Self {
            //     ma: 0.0,
            //     ma_mom: 0.0,
            //     ma_sum: 0.0,
            //     count: 0
            // })
            panic!("")
        }
    }
}

impl Default for VelTrend {
    fn default() -> Self {
        Self::new(9, 3).unwrap()
    }
}