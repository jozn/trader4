use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
// use crate::dc_intel::FrameMem;
use crate::gate_api::{GateWay, NewPosDep};
use crate::{candle, helper};

impl Brain5 {
    // todo fix panic
    pub fn read_pair_meta(&self, si: i64) -> &PairMemory {
        let mut idx = 0;
        let mut found = false;
        for pm in &self.db {
            if pm.pair.to_symbol_id() == si {
                found = true;
                break;
            }
            idx += 1;
        }
        assert!(found);
        let m = self.db.get(idx).unwrap();
        m
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMemory {
        let mut idx = 0;
        let mut found = false;
        for pm in &self.db {
            if pm.pair.to_symbol_id() == si {
                found = true;
                break;
            }
            idx += 1;
        }
        if !found {
            self.db.push(PairMemory::new(
                Pair::id_to_symbol(si),
                &CandleConfig::default(),
            ));
        }
        let m = self.db.get_mut(idx).unwrap();
        m
    }

    // new engine
    pub fn on_price_tick_ne_dc_v4(&mut self, pair: &Pair, tick: Tick) {
        let symbol_id = pair.to_symbol_id();
        let mut pari_mem = self.borrow_pair_meta(symbol_id);
        pari_mem.last_tick = Some(tick.clone());
        let frame_opt = pari_mem.ne3.add_tick(&tick);
        self.update_all_tailing_pos();

        match frame_opt {
            None => {}
            Some(frame) => {
                let nstr = &frame.strength;
                let dc = &frame.dcs;
                let f = &frame;
                let kline_id = f.fid;
                let stop = f.big_low;
                let hl = f.big_high - f.big_low;
                // let hl_pip = (f.big_high - f.big_low) * 10_000.;
                let hl_pip = (f.big_high - f.big_low) * pair.get_pip_multi();
                let med_pip = f.med_dc_hl_pip;

                let profit = stop + 2. * hl / 3.;
                // let profit = f.big_high;

                // if dc.buy1  {
                // if dc.buy1 && dc.vvv.avg_vel_pip >=0. {
                // if dc.buy1 && dc.dir > 0. {
                // if dc.low_sig && dc.rti > 30. {
                // if dc.low_sig  {
                if dc.buy2 {
                    let np = NewPosDep {
                        symbol_id_dep: symbol_id,
                        is_short: false,
                        size_base: 10000,
                        // take_profit_price: profit, // 10 pip
                        // take_profit_price: cal_price(tick.price_raw, med_pip * 2. ), // 10 pip
                        // take_profit_price: cal_price(tick.price_raw, 7.), // 10 pip
                        take_profit_price: pair.cal_price(tick.price_raw, 7.), // 10 pip
                        // stop_loose_price: f.big_low,
                        // stop_loose_price: cal_price(tick.price_raw, - med_pip ),
                        stop_loose_price: pair.cal_price(tick.price_raw, -7.),
                        at_price: tick.price_raw,
                        time_s: tick.time_s,
                        frame_ne2: frame.clone(),
                        // frame: frame.clone(),
                        // ta_med: ta_med.clone(),
                        // ta_big: ta_big.clone(),
                        ..Default::default()
                    };

                    let np = NewPosDep {
                        symbol_id_dep: symbol_id,
                        is_short: false,
                        size_base: 10000,
                        // take_profit_price: cal_price(tick.price_raw, 4.7 ), // 10 pip
                        // take_profit_price: cal_price(tick.price_raw, 14.5), // 10 pip
                        take_profit_price: pair.cal_price(tick.price_raw, 7.5), // 10 pip
                        // take_profit_price: 0., // 10 pip
                        // stop_loose_price: cal_price(tick.price_raw, - 4.7 ),
                        stop_loose_price: pair.cal_price(tick.price_raw, -4.2),
                        // stop_loose_price: cal_price(tick.price_raw, -12.5),
                        // stop_loose_price: cal_price(tick.price_raw, -7.5),
                        at_price: tick.price_raw,
                        time_s: tick.time_s,
                        frame_ne2: frame.clone(),
                        ..Default::default()
                    };

                    // println!("pari {:#?}", &pair);
                    // println!("np {:#?}", &np);

                    if self.already_acted(symbol_id, kline_id) {
                        return;
                    }

                    // println!("Open long {:#?}", np);
                    self.con.open_position_req_new_dep(&np);
                    // if dc.low_sig {
                    // if dc.low_sig && f.trd_ad >=0. {
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                    // self.go_long2(1, frame.fid, &tick, &frame);
                }

                // if dc.sell1 {
                // if dc.sell1 && dc.dir < 0. {
                // if dc.up_sig && dc.dir < 50. {
                if dc.sell2 {
                    // if dc.up_sig && f.trd_ad <=0.  {
                    //     self.go_long2(1, frame.fid, &tick, &frame);
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                }
            }
        }
    }
    /*    // new engine
    pub fn on_price_tick_ne_dc_v3(&mut self, symbol_id: i64, tick: Tick) {
        let mut pari_mem = self.borrow_pair_meta(symbol_id);
        pari_mem.last_tick = Some(tick.clone());
        let frame_opt = pari_mem.ne3.add_tick(&tick);
        self.update_all_tailing_pos();

        match frame_opt {
            None => {}
            Some(frame) => {
                let nstr = &frame.strength;
                let dc = &frame.dcs;
                let f = &frame;
                let kline_id = f.fid;
                let stop = f.big_low;
                let hl = f.big_high - f.big_low;
                let hl_pip = (f.big_high - f.big_low) * 10_000.;
                let med_pip = f.med_dc_hl_pip;

                let profit = stop + 2. * hl / 3.;
                // let profit = f.big_high;

                // if dc.buy1  {
                // if dc.buy1 && dc.vvv.avg_vel_pip >=0. {
                // if dc.buy1 && dc.dir > 0. {
                // if dc.low_sig && dc.rti > 30. {
                // if dc.low_sig  {
                if dc.buy2 {
                    let np = NewPos {
                        symbol_id: 1,
                        is_short: false,
                        size_usd: 10000,
                        // take_profit_price: profit, // 10 pip
                        // take_profit_price: cal_price(tick.price_raw, med_pip * 2. ), // 10 pip
                        take_profit_price: cal_price(tick.price_raw, 7.), // 10 pip
                        // stop_loose_price: f.big_low,
                        // stop_loose_price: cal_price(tick.price_raw, - med_pip ),
                        stop_loose_price: cal_price(tick.price_raw, -7.),
                        at_price: tick.price_raw,
                        time_s: tick.time_s,
                        frame_ne2: frame.clone(),
                        // frame: frame.clone(),
                        // ta_med: ta_med.clone(),
                        // ta_big: ta_big.clone(),
                        ..Default::default()
                    };

                    let np = NewPos {
                        symbol_id: 1,
                        is_short: false,
                        size_usd: 10000,
                        // take_profit_price: cal_price(tick.price_raw, 4.7 ), // 10 pip
                        // take_profit_price: cal_price(tick.price_raw, 14.5), // 10 pip
                        take_profit_price: cal_price(tick.price_raw, 7.5), // 10 pip
                        // take_profit_price: 0., // 10 pip
                        // stop_loose_price: cal_price(tick.price_raw, - 4.7 ),
                        stop_loose_price: cal_price(tick.price_raw, -4.2),
                        // stop_loose_price: cal_price(tick.price_raw, -12.5),
                        // stop_loose_price: cal_price(tick.price_raw, -7.5),
                        at_price: tick.price_raw,
                        time_s: tick.time_s,
                        frame_ne2: frame.clone(),
                        ..Default::default()
                    };

                    if self.already_acted(symbol_id, kline_id) {
                        return;
                    }

                    // println!("Open long {:#?}", np);
                    self.con.open_position_req_new(&np);
                    // if dc.low_sig {
                    // if dc.low_sig && f.trd_ad >=0. {
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                    // self.go_long2(1, frame.fid, &tick, &frame);
                }

                // if dc.sell1 {
                // if dc.sell1 && dc.dir < 0. {
                // if dc.up_sig && dc.dir < 50. {
                if dc.sell2 {
                    // if dc.up_sig && f.trd_ad <=0.  {
                    //     self.go_long2(1, frame.fid, &tick, &frame);
                    //     self.go_short2(1, frame.fid, &tick, &frame);
                }
            }
        }
    }*/
}
