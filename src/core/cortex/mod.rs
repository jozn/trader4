use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

// Flags is db for store of signals, markers,...
#[derive(Debug, Default)]
pub struct FlagsDB {
    flag_id: i32,
    flags_set: HashSet<FlagsRow>,
}

impl FlagsDB {
    pub fn add_once(&mut self, flag_row: &mut FlagsRow) {
        assert_eq!(flag_row.flag_id, 0);
        assert!(flag_row.eng_key.len() > 0);
        assert!(flag_row.type_key.len() > 0);

        self.flag_id += 1;
        flag_row.flag_id = self.flag_id;
        self.flags_set.insert(flag_row.clone());
    }

    pub fn replace(&mut self, flag_row: &FlagsRow) {
        assert!(flag_row.flag_id > 0);
        assert!(flag_row.eng_key.len() > 0);
        assert!(flag_row.type_key.len() > 0);

        self.flags_set.replace(flag_row.clone());
    }

    // todo: add sig_eng key to all fns

    pub fn remove_type(&mut self, medium_bar_id: i32, type_key: &str) {
        // we should have one type per medium_bar but this arr for assuarnce
        let mut arr = vec![];
        for f in self.flags_set.iter() {
            if f.medium_bar_id == medium_bar_id && f.type_key == type_key {
                arr.push(f.clone());
            }
        }
        for f in arr {
            self.flags_set.remove(&f);
        }
    }

    pub fn remove_all(&mut self, medium_bar_id: i32) {
        let mut arr = vec![];
        for f in self.flags_set.iter() {
            if f.medium_bar_id == medium_bar_id {
                arr.push(f.clone());
            }
        }
        for f in arr {
            self.flags_set.remove(&f);
        }
    }

    pub fn get(&self, medium_bar_id: i32, type_key: &str) -> Option<FlagsRow> {
        for f in self.flags_set.iter() {
            if f.medium_bar_id == medium_bar_id && f.type_key == type_key {
                return Some(f.clone());
            }
        }
        None
    }

    // remove
    pub fn exist(&self, medium_bar_id: i32, type_key: &str) -> bool {
        self.get(medium_bar_id, type_key).is_some()
    }
}

fn assert_valid_flag_row(flag_row: &FlagsRow) {
    assert!(flag_row.eng_key.len() > 0);
    assert!(flag_row.type_key.len() > 0);
    assert!(flag_row.medium_bar_id > 0);
    assert!(flag_row.time_sec > 0);
}

// Flags: Signals,...
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct FlagsRow {
    pub flag_id: i32,
    pub eng_key: &'static str,
    pub type_key: &'static str, // Give flexblity to sig_eng without the Enum -- "early_long",..
    // pub flag_type: FlagType, //?
    pub medium_bar_id: i32,
    pub small_bar_id: i32, //?
    pub time_sec: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagType {
    LongEarly,
    LongFinal,
}
