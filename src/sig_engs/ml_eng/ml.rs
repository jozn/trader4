use super::*;
use crate::app;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::{Cortex, CortexRef, FlagsRowCond};
use crate::gate_api::NewPosReq;
use crate::json_output::{JsonMaker, MarkerJson, RowJson, SkyJsonOut};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefMut};
use std::rc::Rc;

// Sky Engine
// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Debug, Clone)]
pub struct MLEng {
    pub cortex: CortexRef,
    pub frames: Vec<MLFrame>,
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

    pub fn on_end(&self) {
        // Note: we use frames only and in json fromat as in debug format the size of file output
        //  too large (>500MB) as all Bars data are on debugs. We ignore Bar info in json format
        // let s = format!("{:#?}", self); // only when needed -- (>500MB) file size
        // todo: can we proude debug format but with serde ignore attrubtes? (check serde)
        let s = serde_json::to_string_pretty(&self.frames).unwrap();
        // println!("{}",s);
        std::fs::write("./debug/runtime/ml_eng_frames_dump_json.txt", s);

        // get DebugDumpBars
        let debug_bars = self.mutli_bars.get_bars_dump(3);
        let s = format!("{:#?}", debug_bars);
        std::fs::write("./debug/runtime/ml_eng_bars_dump.txt", s);
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
                let act = self.set_signals_v1(&tick, &mut frame, &mr);
                // self.set_signals_v1(&tick, &mut frame, &mr);

                let time_bar_med = mr.medium.primary.get_open_time_sec();
                let kid = mr.small.primary.seq;
                let mid = mr.medium.primary.seq - 1;

                // let act = self.cortex_mem.consume_action(time_bar_med);
                // let act = self.cortex_mem.consume_action(0);
                let cor = self.get_cortex();

                if mr.medium_full {
                    // println!("{:?}", act);
                    // todo: make this better - entire memory
                    // frame.signal_mem = self.cortex_mem.get_snapshot(kid);
                    // frame.signal_mem = self.cortex_mem.get_snapshot(0);
                    // frame.signal_action = self.cortex_mem.get_action(time_bar_med);
                    // frame.signal_action = self.cortex_mem.get_action(0);
                    // self.cortex_mem.clear_old(time_bar_med);
                    // let co = self.get_cortex_mut();
                    // co.flags.get_all()
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

                    self.frames.push(frame);
                }
                act
            }
        };
        match sig {
            None => {}
            Some(act) => {
                // println!("time: {}", app::clock::get_clock_time_ms());
                let f = &act;
                let kline_id = f.small_kid;
                let pair = tick.pair.clone();

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
                        // frame: MLFrame::default(),
                        frame: act.frame_insight,
                    };
                    let mut cor = self.get_cortex_mut();
                    let last = cor.get_last_trade(pair);
                    if last.trade_cnt == 0 || last.is_closed {
                        cor.new_positions.push(np);
                    }
                }
            }
        };
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
            let marks = fm.get_frames_markers();
            for m in marks {
                out.push(m);
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
