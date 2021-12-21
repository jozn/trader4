use serde::{Deserialize, Serialize};
use super::postion::*;

// todo remove
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position_BK {
    pub pos_id: u64,
    pub symbol_id: i64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    pub open_time: u64,
    pub open_time_str: String,
    pub open_price: f64,
    pub high_exit_price: f64,
    pub low_exit_price: f64,
    pub close_price: f64,
    pub close_time: u64,
    pub close_time_str: String,
    pub finished: bool, // tod: status
    pub duration: String,
    pub profit: f64,
    pub spread_fees: f64,
    pub final_balance: f64,
    pub touch_low_pip: f64,
    pub touch_high_pip: f64,
    pub locked: f64,

    // Context flat - When rust fixed csv out move it to ctx
    // sm_ prefix: start-medium_
    pub sm_ema: f64,
    pub sm_mom: f64,
    pub sm_roc: f64,
    pub sm_atr: f64,
    pub sm_rsi: f64,
    pub sm_cci: f64,
    pub sm_macd_pip: f64,
    pub sm_fisher: f64,
    pub sm_start_vel_pip: f64,
    pub sm_count: u32,
    pub sm_avg_vel_pip: f64,
    pub sm_end_vel_pip: f64,
    // sb_ prefix: start-big_
    pub sb_ema: f64,
    pub sb_mom: f64,
    pub sb_roc: f64,
    pub sb_atr: f64,
    pub sb_rsi: f64,
    pub sb_cci: f64,
    pub sb_macd_pip: f64,
    pub sb_fisher: f64,
    pub sb_start_vel_pip: f64,
    pub sb_count: u32,
    pub sb_avg_vel_pip: f64,
    pub sb_end_vel_pip: f64,
}