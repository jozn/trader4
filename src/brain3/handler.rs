use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::dc_intel::FrameMem;
use crate::gate_api::GateWay;
use crate::{candle, helper};

impl Brain3 {
    pub fn on_price_tick_no(&mut self, symbol_id: i64, tick: Tick) {
        self.last_tick = Some(tick.clone());

        let frame_opt = self.dc_intl.add_tick(&tick);

        match frame_opt {
            None => {}
            Some(frame) => {
                let dc_str = &frame.dc_strength;

                // reverse
                if dc_str.l_low && dc_str.trend < 0. {
                    // self.go_long(1, frame.fid, &tick, &frame);
                }

                if dc_str.l_low && frame.trd2 > 0. && frame.vel.avg_vel_pip > 0. {
                    // self.go_long(1, frame.fid, &tick, &frame);
                    self.go_short(1, frame.fid, &tick, &frame);
                }

                if dc_str.h_high && frame.trd2 < 0. && frame.vel.avg_vel_pip < 0. {
                    // self.go_short(1, frame.fid, &tick, &frame);
                    self.go_long(1, frame.fid, &tick, &frame);
                }
            }
        }

        // self.ticks_arr.push(tick);
        // let small_tick_size = 400;
        // if self.ticks_arr.len() >= small_tick_size as usize {
        //     self.dc_intl.add_tick(self.ticks_arr.clone());
        //     self.ticks_arr.clear();
        //     // self.on_completed_small_candle(symbol_id);
        //     self.on_completed_small_candle(symbol_id);
        //     self.update_all_tailing_pos();
        // }
    }

    // good result
    pub fn on_price_tick(&mut self, symbol_id: i64, tick: Tick) {
        self.last_tick = Some(tick.clone());

        let frame_opt = self.dc_intl.add_tick(&tick);

        match frame_opt {
            None => {}
            Some(frame) => {
                let dc_str = &frame.dc_strength;

                // reverse
                if dc_str.l_low && dc_str.trend < 0. {
                    // self.go_long(1, frame.fid, &tick, &frame);
                }

                if dc_str.h_high && frame.trd2 > 0. && frame.vel.avg_vel_pip > 0. {
                    // self.go_long(1, frame.fid, &tick, &frame);
                    self.go_short(1, frame.fid, &tick, &frame);
                }

                if dc_str.l_low && frame.trd2 < 0. && frame.vel.avg_vel_pip < 0. {
                    // self.go_short(1, frame.fid, &tick, &frame);
                    self.go_long(1, frame.fid, &tick, &frame);
                }
            }
        }

        // self.ticks_arr.push(tick);
        // let small_tick_size = 400;
        // if self.ticks_arr.len() >= small_tick_size as usize {
        //     self.dc_intl.add_tick(self.ticks_arr.clone());
        //     self.ticks_arr.clear();
        //     // self.on_completed_small_candle(symbol_id);
        //     self.on_completed_small_candle(symbol_id);
        //     self.update_all_tailing_pos();
        // }
    }

    fn on_completed_small_candle(&mut self, symbol_id: i64) {}
}
