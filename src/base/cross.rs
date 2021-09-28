// Licence: A modified version of yata library.
use crate::base::*;
use serde::{Deserialize, Serialize};

type DiffParam = (f64, f64);
pub type SignalsRes = (SimpleCrossEvent, SimpleCrossEvent);

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SimpleCross {
    up: CrossAbove,
    down: CrossUnder,
}

impl SimpleCross {
    pub fn new() -> Self {
        SimpleCross {
            up: Default::default(),
            down: Default::default(),
        }
    }

    pub fn next(&mut self, value1: f64, value2: f64) -> SignalsRes {
        let up = self.up.binary(value1, value2);
        let r_up = SimpleCrossEvent::conv_bull(up);

        let down = self.down.binary(value1, value2);
        let r_down = SimpleCrossEvent::conv_bear(down);

        (r_up, r_down)
    }
}

// Used when needs signal line1 cross above line2.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CrossAbove {
    last_delta: f64,
    is_new: bool,
}

impl CrossAbove {
    fn new() -> Self {
        CrossAbove {
            last_delta: 0.0,
            is_new: true,
        }
    }

    /// Returns `true` when value1 crosses `value2` timeseries upwards
    /// Otherwise returns `false`
    #[inline]
    pub fn binary(&mut self, value1: f64, value2: f64) -> bool {
        let last_delta = self.last_delta;
        let current_delta = value1 - value2;

        self.last_delta = current_delta;

        // Explicitly return (fool proof)
        if self.is_new {
            self.is_new = false;
            return false;
        }

        last_delta < 0. && current_delta >= 0.
    }
}

impl Default for CrossAbove {
    fn default() -> Self {
        Self::new()
    }
}

// Used when needs signal line1 cross below line2.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CrossUnder {
    last_delta: f64,
    is_new: bool,
}

impl CrossUnder {
    fn new() -> Self {
        CrossUnder {
            last_delta: 0.0,
            is_new: true,
        }
    }

    /// Returns `true` when value1 crosses `value2` timeseries downwards
    /// Otherwise returns `false`
    #[inline]
    pub fn binary(&mut self, value1: f64, value2: f64) -> bool {
        let last_delta = self.last_delta;
        let current_delta = value1 - value2;

        self.last_delta = current_delta;

        // Explicitly return (fool proof)
        if self.is_new {
            self.is_new = false;
            return false;
        }

        last_delta > 0. && current_delta <= 0.
    }
}

impl Default for CrossUnder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_above_under() {
        // Line1 starts from below line2 then it cross above and then it cross below and last it cross above line2.
        let t1 = vec![2., 2.5, 3.5, 3.2, 2.5, 3.0, 3.3];
        let t2 = vec![4., 3.5, 2.5, 3.0, 3.5, 3.1, 3.2];
        let r_up = vec![0, 0, 1, 0, 0, 0, 1];
        let r_down = vec![0, 0, 0, 0, 1, 0, 0];

        let mut cross = CrossAbove::new();
        (0..t1.len()).for_each(|i| {
            let v1 = t1[0];
            let v2 = t2[0];
            let res = r_up[0];

            let did = cross.binary(v1, v2);
            assert_eq!(did as i32, res);
        });

        let mut cross = CrossUnder::new();
        (0..t1.len()).for_each(|i| {
            let v1 = t1[0];
            let v2 = t2[0];
            let res = r_down[0];

            let did = cross.binary(v1, v2);
            assert_eq!(did as i32, res);
        });
    }

    #[test]
    fn test_cross_above_yata() {
        // data from yate library
        let t1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let t2 = vec![5.0, 3.0, 1.8, 2.9, 4.1, 5.6];
        let r = vec![0, 0, 0, 0, 1, 0];

        let mut cross = CrossAbove::new();
        (0..t1.len()).for_each(|i| {
            let v1 = t1[0];
            let v2 = t2[0];
            let res = r[0];

            let did = cross.binary(v1, v2);
            assert_eq!(did as i32, res);
        })
    }
}
