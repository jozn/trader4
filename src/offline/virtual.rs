use super::*;
use crate::ta::EMA;
use crate::{helper, ta};

impl BackendEngine {
    pub fn get_signal_power(&mut self, signal_key: &str) -> f64 {
        match self.tails.get(signal_key) {
            None => 0.,
            Some(tw) => tw.last_val,
        }
    }
    pub fn virtual_on_close_position(&mut self, pos: &Position) {
        let pow = if pos.profit > 0. { 1. } else { -1. };
        let o = self.tails.get_mut(&pos.signal_key);
        match o {
            None => {
                let mut tw = TailingWinRate::new();
                tw.next(pow);
                self.tails.insert(pos.signal_key.clone(), tw);
            }
            Some(tw) => {
                tw.next(pow);
            }
        }
    }
}

#[derive(Debug)]
pub struct TailingWinRate {
    pub ema: ta::EMA,
    pub last_val: f64,
    pub pair: u64,
}

impl TailingWinRate {
    fn new() -> Self {
        TailingWinRate {
            ema: ta::EMA::new(5).unwrap(),
            last_val: 0.,
            pair: 0,
        }
    }
    fn next(&mut self, val: f64) -> f64 {
        let ma = self.ema.next(val);
        self.last_val = helper::rond(ma, 3);
        ma
    }
}
