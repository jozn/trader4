pub enum Pair {
    EURUSD,
    GBPUSD,
    USDJPY,
    AUDUSD,
    USDCHF,
    USDCAD,
    NZDUSD,
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

//// Deprecated
pub enum PairDep {
    EURUSD(PairConf),
    GBPUSD(PairConf),
    USDJPY(PairConf),
    AUDUSD(PairConf),
    USDCHF(PairConf),
    USDCAD(PairConf),
    NZDUSD(PairConf),
}
pub fn get_paris_dep() -> Vec<PairDep> {
    let arr = vec![
        PairDep::AUDUSD(get_def_conf_dep(1)),
        PairDep::GBPUSD(get_def_conf_dep(2)),
        PairDep::USDJPY(get_def_conf_dep(4)),
        PairDep::AUDUSD(get_def_conf_dep(5)),
        PairDep::USDCHF(get_def_conf_dep(6)),
        PairDep::USDCAD(get_def_conf_dep(8)),
    ];
    arr
}

pub fn get_def_conf_dep(id: i64) -> PairConf {
    PairConf {
        pair: Pair::EURUSD,
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
