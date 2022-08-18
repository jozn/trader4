pub mod flags;

pub use flags::*;
use std::cell::{Cell, RefCell};

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
    pub orders: f64,
    pub v_orders: f64,
    pub wallet: f64,
    pub policy: f64,
    pub time_table: f64,
}
