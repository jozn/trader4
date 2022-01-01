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
    // new engine
    pub fn on_price_tick_ne_dc_v2(&mut self, symbol_id: i64, tick: Tick) {
        self.last_tick = Some(tick.clone());

        let frame_opt = self.ne.add_tick(&tick);

        match frame_opt {
            None => {}
            Some(frame) => {
                let nstr = &frame.strength;
                let dc = &frame.dcs;
                let f = &frame;

                if dc.low_sig {
                    // if dc.low_sig && f.trd_ad >=0. {
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                    self.go_long2(1, frame.fid, &tick, &frame);
                }

                if dc.up_sig {
                    // if dc.up_sig && f.trd_ad <=0.  {
                    //     self.go_long2(1, frame.fid, &tick, &frame);
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                }
            }
        }
    }

    // new engine
    pub fn on_price_tick_ne_dc_old(&mut self, symbol_id: i64, tick: Tick) {
        self.last_tick = Some(tick.clone());

        let frame_opt = self.ne.add_tick(&tick);

        match frame_opt {
            None => {}
            Some(frame) => {
                let nstr = &frame.strength;
                let dc = &frame.dc;

                if dc.buy {
                    // self.go_short2(1, frame.fid, &tick, &frame);
                    self.go_long2(1, frame.fid, &tick, &frame);
                }

                if dc.sell {
                    // self.go_long2(1, frame.fid, &tick, &frame);
                    self.go_short2(1, frame.fid, &tick, &frame);
                }
            }
        }
    }
}
