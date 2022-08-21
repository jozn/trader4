use super::*;
use crate::app;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::{Cortex, CortexRef};
use crate::cortex_old::eng_memory::CortexMem;
use crate::cortex_old::types::{ActionSignal, SignalMem};
use crate::gate_api::NewPosReq;
use crate::json_output::{JsonMaker, MarkerJson, RowJson, SkyJsonOut};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefMut;
use std::rc::Rc;

// Sky Engine
// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Debug, Clone)]
pub struct MLEng {
    pub cortex_mem: CortexMem,
    pub cortex: CortexRef,
    pub frames: Vec<MLFrame>,
    pub mutli_bars: MultiBars,
}

impl MLEng {
    pub fn new(pair: &Pair, cortex_ref: CortexRef) -> Self {
        MLEng {
            cortex_mem: CortexMem::new(),
            cortex: cortex_ref,
            frames: vec![],
            mutli_bars: MultiBars::new(pair),
        }
    }

    pub fn get_mut_cortex(&mut self) -> RefMut<Cortex> {
        self.cortex.as_ref().borrow_mut()
    }

    pub fn add_tick(&mut self, tick: &BTickData) -> Option<ActionSignal> {
        let mul_res = self.mutli_bars.add_tick(tick);
        // let mut m = Rc::get_mut(&mut self.cortex).unwrap();
        // m.orders +=1.0;
        // self.cortex.orders += 1.0;

        // let m = self.cortex.as_ptr();
        // let mut m = self.cortex.as_ref().borrow_mut();
        // let mut m = self.cortex.as_ref().get_mut();
        // let  m = self.cortex.as_ref();
        // let mut m = self.cortex.as_ref().borrow_mut();
        // // let m = (*m);
        // // let mut x = m.get_mut();
        // // let mut x = self.cortex.get_mut();
        // // println!("ord: {}", self.cortex.borrow_mut().);
        // println!("ord: {:#?}", m);
        // m.orders += 1.0;
        // m.flags.remove_flags(vec![]);
        // drop(m);
        // println!("exxx: {:#?}", m.orders);
        // self
        // m. += 1.0;
        // println!("ord: {}", self.cortex.borrow_mut().);

        // let bb = self.mutli_bars.medium_bars.bars_primary.last().unwrap();
        // let time_bar_med = bb.primary.get_open_time_sec();
        // let kid = self.mutli_bars.medium_bars.primary_seq;

        let sig = match mul_res {
            None => None,
            Some(mr) => {
                let mut frame = new_frame(&mr);
                self.set_signals_random1(&tick, &mut frame, &mr);
                // self.set_signals_v1(&tick, &mut frame, &mr);

                let time_bar_med = mr.medium.primary.get_open_time_sec();
                let kid = mr.small.primary.seq;

                let act = self.cortex_mem.consume_action(time_bar_med);
                // let act = self.cortex_mem.consume_action(0);

                if mr.medium_full {
                    // println!("{:?}", act);
                    // todo: make this better - entire memory
                    frame.signal_mem = self.cortex_mem.get_snapshot(kid);
                    // frame.signal_mem = self.cortex_mem.get_snapshot(0);
                    frame.signal_action = self.cortex_mem.get_action(time_bar_med);
                    // frame.signal_action = self.cortex_mem.get_action(0);
                    self.cortex_mem.clear_old(time_bar_med);

                    self.frames.push(frame);

                    // Temp
                    /*match act.clone() {
                        None => {}
                        Some(act) => {
                            println!("time: {}", app::clock::get_clock_time_ms());

                            let f = &act;
                            let kline_id = f.small_kid;
                            let pair = tick.pair.clone();

                            if act.long {
                                // if self.already_acted(symbol_id, kline_id) {
                                //
                                // }

                                let np = NewPosReq {
                                    pair: tick.pair.clone(),
                                    is_short: false,
                                    // base_asset_size: 10_000.0,
                                    base_asset_size: 10.0,
                                    // base_asset_size: 100.0,
                                    exit_high_price: pair.cal_price(tick.bid_price, act.profit),
                                    exit_low_price: pair.cal_price(tick.bid_price, act.loss),
                                    virtual_id: 1, //self.sim_virtual.next_virtual_id(), // todo
                                    is_virtual: false, // todo tailing
                                    signal_key: "sky_1".to_string(),
                                    at_price: tick.ask_price,
                                    time_sec: tick.timestamp_sec as u64,
                                    // frame: MLFrame::default(),
                                    frame: act.frame_insight,
                                };
                                let mut cor = self.get_mut_cortex();
                                if app::helper::get_rand(100) > 0 {
                                    cor.new_positions.push(np);
                                }
                            }
                        }
                    };*/
                }
                act
            }
        };
        let sig2 = sig.clone();
        match sig {
            None => {}
            Some(act) => {
                println!("time: {}", app::clock::get_clock_time_ms());

                let f = &act;
                let kline_id = f.small_kid;
                let pair = tick.pair.clone();

                if act.long {
                    // if self.already_acted(symbol_id, kline_id) {
                    //
                    // }

                    let np = NewPosReq {
                        pair: tick.pair.clone(),
                        is_short: false,
                        // base_asset_size: 10_000.0,
                        base_asset_size: 10.0,
                        // base_asset_size: 100.0,
                        exit_high_price: pair.cal_price(tick.bid_price, act.profit),
                        exit_low_price: pair.cal_price(tick.bid_price, act.loss),
                        virtual_id: 1,     //self.sim_virtual.next_virtual_id(), // todo
                        is_virtual: false, // todo tailing
                        signal_key: "sky_1".to_string(),
                        at_price: tick.ask_price,
                        time_sec: tick.timestamp_sec as u64,
                        // frame: MLFrame::default(),
                        frame: act.frame_insight,
                    };
                    let mut cor = self.get_mut_cortex();
                    let last = cor.get_last_trade(pair);
                    if last.trade_cnt == 0 || last.is_closed {
                        cor.new_positions.push(np);
                    }
                    if app::helper::get_rand(100) > 90 {
                        // cor.new_positions.push(np);
                    }
                }
            }
        };
        sig2
    }
}

impl JsonMaker for MLEng {
    fn get_bars(&self) -> MultiBars {
        self.mutli_bars.clone()
    }

    fn get_markers(&self, start: i64, end: i64) -> Vec<MarkerJson> {
        let mut out = vec![];
        for fm in &self.frames {
            let bar = &fm.info.bar_medium.primary;
            if !(bar.open_time >= start && bar.open_time <= end) {
                continue;
            }
            // Markers
            if fm.get_early_mark().is_some() {
                out.push(fm.get_early_mark().unwrap());
            }
            if fm.get_long_final_mark().is_some() {
                out.push(fm.get_long_final_mark().unwrap());
            }
        }
        // println!("markers {:?}",out);
        out
    }

    fn set_json_data(&self, jo: &mut SkyJsonOut) {
        for fm in &self.frames {
            let bar = &fm.info.bar_medium.primary;
            // todo: fix this
            // if !(bar.open_time >= start && bar.open_time <= end) {
            //     continue;
            // }
            let time = bar.open_time / 1000;
            let score = &fm.score;
            // Add scores
            //  let score = &fm.tscore;
            jo.score_bull.push(RowJson {
                time,
                value: score.bull as f64,
            });
            jo.score_bear.push(RowJson {
                time,
                value: score.bear as f64,
            });
            jo.score_diff.push(RowJson {
                time,
                value: score.diff as f64,
            });
        }
    }
}
