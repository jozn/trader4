use super::*;
use crate::candle::CandleConfig;
use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;

// static ALL_SYMBOLS: OnceCell<HashMap<i64,TSymbol>> = OnceCell::new();

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
    // NONE, // We have in here for some error in symbol_id convertion
    EURUSD,
    GBPUSD,
    USDJPY,
    AUDUSD,
    USDCHF,
    USDCAD,
    NZDUSD,
}

impl Pair {
    /*pub fn to_symbol_id_dep(&self) -> i64 {
        use Pair::*;
        match self {
            EURUSD => 1,
            GBPUSD => 2,
            USDJPY => 4,
            AUDUSD => 5,
            USDCHF => 6,
            USDCAD => 8,
            NZDUSD => 12,
            // NONE => 0,
        }
    }*/

    pub fn to_symbol_id(&self) -> i64 {
        let name = self.to_string();
        let sym = get_pepperstone_symbol_name(&name);
        sym.symbol_id
    }

    pub fn get_conf(&self) -> TSymbol {
        let name = self.to_string();
        get_pepperstone_symbol_name(&name)
    }
    /*
        pub fn get_pip_old(&self) -> i64 {
            use Pair::*;
            match self {
                EURUSD | GBPUSD | AUDUSD | USDCHF | USDCAD | NZDUSD => 10_000,
                USDJPY => 10_00,
            }
        }
    */
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

////////// Deprece4ated: Save fot Delte////
/*#[derive(Debug, Default)]
pub struct PairCfg {
    pub symbol_id: i64,
    // pub active: bool,
    pub pip: u32,
    pub change: u32, // Fracation change ex: EuroUsd: 5 means 5 fraction of price change 0.00001
}

pub struct PairConfDep {
    pub pair: Pair,
    pub symbol_id: i64,
    pub active: bool,
    pub small_size: u64,
    pub medium_size: u64,
    pub big_size: u64,
    pub trade_size_xlot: u64,  // 100 = 1lot - 1 = 0.01 lot
    pub take_profit_xpip: u64, // 10 = 1pip
    pub stop_loose_xpip: u64,
}

pub fn get_pairs() -> Vec<PairConfDep> {
    use Pair::*;

    let arr = vec![
        get_def_conf_dep(AUDUSD, 1),
        get_def_conf_dep(GBPUSD, 2),
        get_def_conf_dep(USDJPY, 4),
        get_def_conf_dep(AUDUSD, 5),
        get_def_conf_dep(USDCHF, 6),
        get_def_conf_dep(USDCAD, 8),
    ];
    arr
}

pub fn get_def_conf(pair: Pair) -> PairConfDep {
    PairConfDep {
        pair: pair.clone(),
        symbol_id: pair.to_symbol_id(),
        active: true,
        small_size: 3,
        medium_size: 3,
        big_size: 3,
        trade_size_xlot: 10, // 10_000$
        take_profit_xpip: 100,
        stop_loose_xpip: 100,
    }
}

pub fn get_def_conf_dep(pair: Pair, id: i64) -> PairConfDep {
    PairConfDep {
        pair: pair,
        symbol_id: id,
        active: true,
        small_size: 3,
        medium_size: 3,
        big_size: 3,
        trade_size_xlot: 10, // 10_000$
        take_profit_xpip: 100,
        stop_loose_xpip: 100,
    }
}
*/
