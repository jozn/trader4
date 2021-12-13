use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use std::fmt::format;

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
    pub fn to_symbol_id(&self) -> i64 {
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
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

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
    let a = get_paris();
    let ids: Vec<i64> = Pair::into_enum_iter().map(|p| p.to_symbol_id()).collect();
    ids
}

pub fn get_all_symbols() -> Vec<Pair> {
    let a = get_paris();
    let pairs: Vec<Pair> = Pair::into_enum_iter().map(|p| p).collect();
    pairs
}

pub struct PairConf {
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

pub fn get_paris() -> Vec<PairConf> {
    use Pair::*;

    let arr = vec![
        get_def_conf(AUDUSD, 1),
        get_def_conf(GBPUSD, 2),
        get_def_conf(USDJPY, 4),
        get_def_conf(AUDUSD, 5),
        get_def_conf(USDCHF, 6),
        get_def_conf(USDCAD, 8),
    ];
    arr
}

pub fn get_def_conf(pair: Pair, id: i64) -> PairConf {
    PairConf {
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
