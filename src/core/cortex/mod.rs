pub mod flags;
pub mod pair_mem;

pub use flags::*;
pub use pair_mem::*;
use std::cell::{Cell, RefCell};

use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

// In order to get mut of this use:
//   let mut m = self.cortex.as_ref().borrow_mut();
pub type CortexRef = Rc<RefCell<Cortex>>;

pub fn new_cortex_ref() -> CortexRef {
    Rc::new(RefCell::new(Cortex::default()))
}

// Specs:
//  no gateway in cortex - only brain

#[derive(Debug, Default)]
pub struct Cortex {
    pub time_ms: i64, // set in every small candle in brain
    pub flags: FlagsDB,
    pub pairs_mem: Vec<PairMemory>,
    pub orders: f64,
    pub v_orders: f64,
    pub wallet: f64,
    pub policy: f64,
    pub time_table: f64,
}

impl Cortex {
    fn init_pair(&mut self, pair: &Pair) {
        if !self.pairs_mem.iter().any(|ps| &ps.pair == pair) {
            self.pairs_mem.push(PairMemory {
                pair: Default::default(),
                last_tick: None,
            })
        }
    }

    pub fn on_price_tick(&mut self, pair: &Pair, tick: BTickData) {
        self.init_pair(pair);

        for pm in self.pairs_mem.iter_mut() {
            if &pm.pair == pair {
                pm.last_tick = Some(tick.clone());
            }
        }
    }
}
