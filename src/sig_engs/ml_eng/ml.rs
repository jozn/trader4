use super::*;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex_old::eng_memory::CortexMem;
use crate::cortex_old::types::{ActionSignal, SignalMem};
use crate::json_output::{JsonMaker, MarkerJson, RowJson, SkyJsonOut};
use serde::{Deserialize, Serialize};

// Sky Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MLEng {
    pub cortex_mem: CortexMem,
    pub frames: Vec<MLFrame>,
    pub mutli_bars: MultiBars,
}

impl MLEng {
    pub fn new(pair: &Pair) -> Self {
        MLEng {
            cortex_mem: CortexMem::new(),
            frames: vec![],
            mutli_bars: MultiBars::new(pair),
        }
    }

    pub fn add_tick(&mut self, tick: &BTickData) -> Option<ActionSignal> {
        let mul_res = self.mutli_bars.add_tick(tick);

        // let bb = self.mutli_bars.medium_bars.bars_primary.last().unwrap();
        // let time_bar_med = bb.primary.get_open_time_sec();
        // let kid = self.mutli_bars.medium_bars.primary_seq;

        match mul_res {
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
                }
                act
            }
        }
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
