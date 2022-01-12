use super::*;
use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref ALL_SYMBOLS_NAME: HashMap<String, TSymbol> = {
        let symbs = super::gen::pepperstone::get_symbols_list();
        let mut m = HashMap::new();
        for s in symbs {
            m.insert(s.name.to_string(), s);
        }
        m
    };
    static ref ALL_SYMBOLS_ID: HashMap<i64, TSymbol> = {
        let symbs = super::gen::pepperstone::get_symbols_list();
        let mut m = HashMap::new();
        for s in symbs {
            m.insert(s.symbol_id, s);
        }
        m
    };
}

fn get_pepperstone_symbol_name(name: &str) -> TSymbol {
    let s = ALL_SYMBOLS_NAME
        .get(name)
        .expect("Symbol not found in symbols map.");
    s.clone()
}

fn get_pepperstone_symbol(sid: i64) -> TSymbol {
    let s = ALL_SYMBOLS_ID
        .get(&sid)
        .expect("Symbol not found in symbols map.");
    s.clone()
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoEnumIterator, PartialEq)]
pub enum Pair {
    EURUSD,
    GBPUSD,
    USDJPY,
    AUDUSD,
    USDCHF,
    USDCAD,
    NZDUSD,
}

impl Pair {
    pub fn to_symbol_id(&self) -> i64 {
        let name = self.to_string();
        let sym = get_pepperstone_symbol_name(&name);
        sym.symbol_id
    }

    pub fn get_conf(&self) -> TSymbol {
        let name = self.to_string();
        get_pepperstone_symbol_name(&name)
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    // todo: update?
    pub fn id_to_symbol(id: i64) -> Self {
        use Pair::*;
        let r = Pair::into_enum_iter().find(|p| p.to_symbol_id() == id);
        match r {
            None => panic!("Could not find symbol id {}", id),
            Some(p) => p,
        }
    }
}

pub fn get_all_symbols_ids() -> Vec<i64> {
    let ids: Vec<i64> = Pair::into_enum_iter().map(|p| p.to_symbol_id()).collect();
    ids
}

pub fn get_all_symbols() -> Vec<Pair> {
    let pairs: Vec<Pair> = Pair::into_enum_iter().map(|p| p).collect();
    pairs
}
