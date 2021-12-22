use super::*;
use crate::gate_api::NewPos;
use serde::{Deserialize, Serialize};

// Note: maybe a custom serde Serializer would be better to extract both medium and big into one struct.

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PositionTA {
    // Context flat - When rust fixed csv out move it to ctx
    // Medium time frame sm_ prefix: start-medium_
    pub sm_mom: f64,
    pub sm_roc: f64,
    pub sm_atr: f64,
    pub sm_rsi: f64,
    pub sm_cci: f64,
    pub sm_macd_pip: f64,
    pub sm_fisher: f64,

    pub sm_1_vel_ma: f64,
    pub sm_1_start_vel_pip: f64,
    pub sm_1_count: u32,
    pub sm_1_avg_vel_pip: f64,
    pub sm_1_end_vel_pip: f64,
    pub sm_2_vel_ma: f64,
    pub sm_2_start_vel_pip: f64,
    pub sm_2_count: u32,
    pub sm_2_avg_vel_pip: f64,
    pub sm_2_end_vel_pip: f64,

    // Big time frame- sb_ prefix: start-big_
    pub sb_mom: f64,
    pub sb_roc: f64,
    pub sb_atr: f64,
    pub sb_rsi: f64,
    pub sb_cci: f64,
    pub sb_macd_pip: f64,
    pub sb_fisher: f64,

    pub sb_1_vel_ma: f64,
    pub sb_1_start_vel_pip: f64,
    pub sb_1_count: u32,
    pub sb_1_avg_vel_pip: f64,
    pub sb_1_end_vel_pip: f64,
    pub sb_2_vel_ma: f64,
    pub sb_2_start_vel_pip: f64,
    pub sb_2_count: u32,
    pub sb_2_avg_vel_pip: f64,
    pub sb_2_end_vel_pip: f64,
}

impl PositionTA {
    pub fn set_techichal_anylse(&mut self, p: &NewPos) {
        // ================ For Start Medium Time Frame =============== //
        let t = &p.ta_med;

        self.sm_mom = t.mom;
        self.sm_roc = t.roc;
        self.sm_atr = t.atr;
        self.sm_rsi = t.rsi;
        self.sm_cci = t.cci;
        self.sm_macd_pip = t.macd.macd_pip;
        self.sm_fisher = t.fisher.fisher;

        let vel_m1 = &t.vel1;
        self.sm_1_vel_ma = vel_m1.ma;
        self.sm_1_start_vel_pip = vel_m1.start_vel_pip;
        self.sm_1_count = vel_m1.count;
        self.sm_1_avg_vel_pip = vel_m1.avg_vel_pip;
        self.sm_1_end_vel_pip = vel_m1.end_vel_pip;

        let vel_m2 = &t.vel2;
        self.sm_2_vel_ma = vel_m2.ma;
        self.sm_2_start_vel_pip = vel_m2.start_vel_pip;
        self.sm_2_count = vel_m2.count;
        self.sm_2_avg_vel_pip = vel_m2.avg_vel_pip;
        self.sm_2_end_vel_pip = vel_m2.end_vel_pip;

        // ================ For Start Big Time Frame =============== //
        // Set big time frame TA
        let t = &p.ta_big;

        self.sb_mom = t.mom;
        self.sb_roc = t.roc;
        self.sb_atr = t.atr;
        self.sb_rsi = t.rsi;
        self.sb_cci = t.cci;
        self.sb_macd_pip = t.macd.macd_pip;
        self.sb_fisher = t.fisher.fisher;

        let vel_b1 = &t.vel1;
        self.sb_1_vel_ma = vel_b1.ma;
        self.sb_1_start_vel_pip = vel_b1.start_vel_pip;
        self.sb_1_count = vel_b1.count;
        self.sb_1_avg_vel_pip = vel_b1.avg_vel_pip;
        self.sb_1_end_vel_pip = vel_b1.end_vel_pip;

        let vel_b2 = &t.vel2;
        self.sb_2_vel_ma = vel_b2.ma;
        self.sb_2_start_vel_pip = vel_b2.start_vel_pip;
        self.sb_2_count = vel_b2.count;
        self.sb_2_avg_vel_pip = vel_b2.avg_vel_pip;
        self.sb_2_end_vel_pip = vel_b2.end_vel_pip;
    }
}
