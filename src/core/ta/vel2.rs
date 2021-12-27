use serde::{Deserialize, Serialize};

use super::*;
use crate::base::*;
use std::collections::VecDeque;

// Vel2 is the proxy for Vel with filed added suffix 2 to support csv output

// Velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vel2 {
    vel: Vel,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VelRes2 {
    pub v2_ma: f64,
    pub v2_start_vel_pip: f64,
    pub v2_count: u32, // all positive/negative candles counts
    pub v2_avg_vel_pip: f64,
    pub v2_end_vel_pip: f64,
}

impl Vel2 {
    pub fn new(ema_period: usize) -> TAResult<Self> {
        match ema_period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                vel: Vel::new(ema_period)?,
            }),
        }
    }

    pub fn next(&mut self, price: f64) -> VelRes2 {
        let v = self.vel.next(price);

        VelRes2 {
            v2_ma: v.ma,
            v2_start_vel_pip: v.start_vel_pip,
            v2_count: v.count,
            v2_avg_vel_pip: v.avg_vel_pip,
            v2_end_vel_pip: v.end_vel_pip,
        }
    }
}

impl Default for Vel2 {
    fn default() -> Self {
        Self::new(99).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
