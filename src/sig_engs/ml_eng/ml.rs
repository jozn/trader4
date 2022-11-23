use super::*;
use crate::app;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::{ActionSignal, Cortex, CortexRef, FlagsRowCond};
use crate::gate_api::NewPosReq;
use crate::json_output::{JsonMaker, MarkerJson, RowJson, SkyJsonOut};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefMut};
use std::ops::Deref;
use std::rc::Rc;

// Sky Engine
#[derive(Debug, Clone)]
pub struct MLEng {
    pub cortex: CortexRef,
    pub frames: Vec<MLFrameRef>,
    pub mutli_bars: MultiBars,
}

impl MLEng {
    pub fn new(pair: &Pair, cortex_ref: CortexRef) -> Self {
        MLEng {
            cortex: cortex_ref,
            frames: vec![],
            mutli_bars: MultiBars::new(pair),
        }
    }

    pub(super) fn get_cortex(&mut self) -> Ref<Cortex> {
        self.cortex.as_ref().borrow()
    }

    pub(super) fn get_cortex_mut(&mut self) -> RefMut<Cortex> {
        self.cortex.as_ref().borrow_mut()
    }

    pub fn add_tick(&mut self, tick: &BTickData) {
        let mul_res = self.mutli_bars.add_tick(tick);
        let pair = tick.pair;

        let sig = match mul_res {
            None => None,
            Some(mr) => {
                let mut frame = new_frame(&mr);

                // let act = self.set_signals_random1(&tick, &mut frame, &mr);
                // let act = self.set_signals_for_ml_v1(&tick, &mut frame, &mr);
                // let act = None;
                // let act = self.set_signals_v1(&tick, &mut frame, &mr);
                let act = self.set_signals_random_for_showcase(&tick, &mut frame, &mr);
                // let act = self.set_signals_random2(&tick, &mut frame, &mr);

                let time_bar_med = mr.medium.primary.get_open_time_sec();
                let kid = mr.small.primary.seq;
                let mid = mr.medium.primary.seq - 1;

                let cor = self.get_cortex();

                if mr.medium_full {
                    let sigs = cor.flags.get_all(&FlagsRowCond {
                        pair,
                        eng_key: ML_ENG,
                        type_key: "ALL",
                        medium_bar_id: Some(mid as i32),
                        // small_bar_id: Some(kid as i32),
                        small_bar_id: None,
                        from_time_sec: None,
                    });
                    drop(cor);
                    frame.signals = sigs;

                    let fr = Rc::new(frame);
                    self.frames.push(fr);
                }
                act
            }
        };
        match sig {
            None => {}
            Some(act) => {
                self.send_new_pos(act, tick);
            }
        };
    }

    fn send_new_pos(&mut self, act: ActionSignal, tick: &BTickData) {
        // println!("time: {}", app::clock::get_clock_time_ms());
        let f = &act;
        let kline_id = f.small_kid;
        let pair = tick.pair.clone();

        let cortex = self.get_cortex();
        let last_trade = cortex.get_last_trade(tick.pair);

        // skip timing -- evey 20 min one
        if last_trade.trade_cnt > 0
            && last_trade.open_time + 10 * 60 > app::clock::get_clock_time_sec()
        {
            // println!("skiping trade {:?}", last_trade);
            return;
        }
        drop(cortex);

        if act.long {
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
                frame_ml_ref: Rc::clone(self.frames.last().unwrap()),
                // frame_ml_ref: Rc::new(MLFrame::default()),
            };
            let mut cor = self.get_cortex_mut();
            let last = cor.get_last_trade(pair);

            // Temp
            // cor.new_positions.push(np);

            let now = app::clock::get_clock_time_sec();
            // Real - every 15min one trade at most
            if last.trade_cnt == 0 || ( last.is_closed && last.open_time + 900 < now) {
                cor.new_positions.push(np);
            }
        }
    }

    // Called when last tick is called - outputting runtime debug data
    pub fn on_end(&self) {
        // Note: we use frames only and in json fromat as in debug format the size of file output
        //  too large (>500MB) as all Bars data are on debugs. We ignore Bar info in json format
        // let s = format!("{:#?}", self); // only when needed -- (>500MB) file size
        // todo: can we proude debug format but with serde ignore attrubtes? (check serde)

        // Only 1000 first frames for reduce debug size
        let mut frams = vec![];
        for (id, f) in self.frames.iter().enumerate() {
            if id < 1000 {
                frams.push(f.deref().clone());
            } else {
                break;
            }
        }
        let s = serde_json::to_string_pretty(&frams).unwrap();
        // println!("{}",s);
        std::fs::write("./debug/runtime/ml_eng_frames_dump_json.txt", s);

        // get DebugDumpBars
        let debug_bars = self.mutli_bars.get_bars_dump(3);
        let s = format!("{:#?}", debug_bars);
        std::fs::write("./debug/runtime/ml_eng_bars_dump.txt", s);
    }
}
