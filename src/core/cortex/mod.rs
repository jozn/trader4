pub mod flags;

pub use flags::*;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub type CortexRef = Rc<Cortex>;

// Specs:
//  no gateway in cortex - only brain

pub struct Cortex {
    pub time_ms: i64, // set in every small candle in brain
    pub flags: FlagsDB,
    pub orders: f64,
    pub v_orders: f64,
    pub wallet: f64,
    pub policy: f64,
    pub time_table: f64,
}
