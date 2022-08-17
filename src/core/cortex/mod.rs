pub mod flags;

pub use flags::*;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub type CortexRef = Rc<Cortex>;

pub fn new_cortex_ref() -> CortexRef {
    Rc::new(Cortex::default())
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
