use crate::configs::assets::Pair;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Flags is db for store of signals, markers,...
#[derive(Debug, Default)]
pub struct FlagsDB {
    flag_id_cnt: i32,
    // flags_set: HashSet<FlagsRow>,
    flags_set: HashMap<String,FlagsRow>,
    flags_archive: Vec<FlagsRow>,
}

impl FlagsDB {
    pub fn add_once_small(&mut self, flag_row: &FlagsRow) -> FlagsRow {
        let mut flag_row = flag_row.clone();
        assert_eq!(flag_row.flag_id, 0);
        assert_valid_flag_row(&flag_row);
        // Only one flag in each small_bar and
        let cond = FlagsRowCond {
            pair: flag_row.pair,
            eng_key: flag_row.eng_key,
            type_key: flag_row.type_key,
            medium_bar_id: Some(flag_row.medium_bar_id),
            small_bar_id: Some(flag_row.small_bar_id),
            // small_bar_id: None, // for unique in medium
            from_time_sec: None,
        };
        let past_flag = self.get(&cond);
        if past_flag.is_some() {
            return past_flag.unwrap();
        }

        self.flag_id_cnt += 1;
        flag_row.flag_id = self.flag_id_cnt;
        self.flags_set.insert(flag_row.flag_key(),flag_row.clone());
        flag_row
    }

    pub fn replace(&mut self, flag_row: &FlagsRow) {
        assert!(flag_row.flag_id > 0);
        assert_valid_flag_row(flag_row);

        self.flags_set.insert(flag_row.flag_key(),flag_row.clone());
    }

    pub fn get_all(&self, param: &FlagsRowCond) -> Vec<FlagsRow> {
        let mut arr = vec![];
        for (_,f )in self.flags_set.iter() {
            if f.eng_key == f.eng_key || param.eng_key == "ALL" {
                if f.type_key == f.type_key || param.type_key == "ALL" {
                    let valid_med = valid_equal_id(param.medium_bar_id, f.medium_bar_id);
                    let valid_small = valid_equal_id(param.small_bar_id, f.small_bar_id);

                    // todo: we need current time
                    let valid_time = match param.from_time_sec {
                        None => true,
                        Some(time) => {
                            // fix
                            if f.time_sec == time {
                                true
                            } else {
                                false
                            }
                        }
                    };

                    if valid_med && valid_small && valid_time {
                        arr.push(f.clone());
                    }
                }
            }
        }
        arr.sort_by_key(|f| f.flag_id);
        arr
    }

    pub fn get(&self, param: &FlagsRowCond) -> Option<FlagsRow> {
        let arr = self.get_all(param);
        arr.last().cloned()
    }

    pub fn remove_cond(&mut self, param: &FlagsRowCond) {
        let flags = self.get_all(param);
        let fids: Vec<i32> = flags.iter().map(|f| f.flag_id).collect();
        self.remove_flags(fids);
    }

    pub fn remove_flag(&mut self, flag: i32) {
        self.remove_flags(vec![flag])
    }

    pub fn remove_flags(&mut self, flags: Vec<i32>) {
        let mut arr = vec![];
        for (fk,f) in self.flags_set.iter() {
            for fid in flags.iter() {
                if f.flag_id == *fid {
                    arr.push(f.clone());
                }
            }
        }
        for f in arr {
            self.flags_set.remove(&f.flag_key());
            self.flags_archive.push(f);
        }
    }
}

fn assert_valid_flag_row(flag_row: &FlagsRow) {
    assert!(flag_row.eng_key.len() > 0);
    assert!(flag_row.type_key.len() > 0);
    assert!(flag_row.medium_bar_id > 0);
    assert!(flag_row.time_sec > 0);
}

// Checks to see if condation is right
fn valid_equal_id(id_opt: Option<i32>, id: i32) -> bool {
    match id_opt {
        None => true,
        Some(mid) => {
            if id == mid {
                true
            } else {
                false
            }
        }
    }
}

// Flags: Signals,...
// #[derive(Debug, Clone, Default, Serialize, Eq, Hash, PartialEq)]
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlagsRow {
    pub flag_id: i32,
    pub pair: Pair,
    pub eng_key: &'static str,
    pub type_key: &'static str, // Give flexblity to sig_eng without the Enum -- "early_long",..
    // pub flag_type: FlagType, //?
    pub medium_bar_id: i32,
    pub small_bar_id: i32, //?
    pub time_sec: i64,
    pub ttl: i64, // time to live
}

impl FlagsRow {
    fn flag_key(&self) -> String {
        let s = self;
        format!(
            "{:?}_{}_{}_{}_{}",
            s.pair, s.eng_key, s.type_key, s.medium_bar_id, s.small_bar_id
        )
    }
}

pub struct FlagsRowCond {
    // We should always set keys -- the reason we do not use Option is that we should
    //  always use them; use the keyword "ALL" for explict igonres of them
    pub pair: Pair,
    pub eng_key: &'static str,
    pub type_key: &'static str, // Option<str>,
    pub medium_bar_id: Option<i32>,
    pub small_bar_id: Option<i32>,
    pub from_time_sec: Option<i64>, // todo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagType {
    LongEarly,
    LongFinal,
}
