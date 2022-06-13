use super::*;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::eng_memory::CortexMem;
use crate::cortex::types::{ActionSignal, SignalMem};
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
    pub fn add_tick(&mut self, tick: &BTickData) {
        let mul_res = self.mutli_bars.add_tick(tick);
        match mul_res {
            None => {}
            Some(mr) => {
                let mut frame = new_frame(&mr);

                if mr.medium_full {
                    self.frames.push(frame);
                }
            }
        }
    }
}
