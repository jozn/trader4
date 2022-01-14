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

// todo: Write a macro to seperate each Pari with catefory and then merge them
//  in Pair.
#[derive(Debug, Serialize, Deserialize, Clone, IntoEnumIterator, PartialEq)]
pub enum Pair {
    // Forex -
    // "FX Majors" - 6
    EURUSD,
    GBPUSD,
    USDJPY,
    AUDUSD,
    USDCHF,
    USDCAD,

    // "FX Minors" - 10
    AUDCAD,
    AUDCHF,
    AUDNZD,
    AUDSGD,
    EURAUD,
    EURCHF,
    EURGBP,
    GBPAUD,
    GBPCHF,
    NZDUSD, // In first assets list

    // "FX Crosses" - 14 list - maybe

    // "FX Exotics"
    USDNOK,
    USDMXN,
    USDSGD,
    USDSEK,
    EURSEK,
    GBPSGD,
    EURNOK,
    EURHUF,
    USDPLN,
    USDDKK,
    GBPNOK,
    CHFSGD,
    EURCZK,
    EURDKK,
    EURHKD,
    EURPLN,
    EURSGD,
    EURTRY,
    EURZAR,
    GBPDKK,
    GBPSEK,
    GBPTRY,
    NOKJPY,
    NOKSEK,
    SEKJPY,
    USDCZK,
    USDHKD,
    USDTRY,
    USDZAR,
    EURMXN,
    USDHUF,
    USDRUB,
    USDCNH,
    USDTHB,
    ZARJPY,
    GBPMXN,
    GBPZAR,

    // Crypto 19
    BTCUSD,
    ETHUSD,
    LTCUSD,
    DASHUSD,
    Ripple,
    BitcoinCash,
    BCHUSD,
    DOTUSD,
    LINKUSD,
    XLMUSD,
    ETHBTC,
    XRPUSD,
    UNIUSD,
    DOGEUSD,
    ADAUSD,
    BNBUSD,
    XAUBTC,
    EOSUSD,
    XTZUSD,

    // Indices 30
    AUS200,
    EUSTX50,
    FRA40,
    GER40,
    HK50,
    IT40,
    JPN225,
    AEX,
    WIG20,
    SPA35,
    SMI,
    UK100,
    US2000,
    US500,
    NAS100,
    US30,
    CN50,
    SCI25,
    VIX,
    Crypto10,
    Crypto20,
    Crypto30,
    CA60,
    CHINAH,
    MidDE50,
    NETH25,
    NOR25,
    SA40,
    SWI20,
    GERTEC30,

    // Energies  7
    XBRUSD,
    XTIUSD,
    XNGUSD,
    SpotBrent,
    SpotCrude,
    NatGas,
    Gasoline,

    //Metals (Spot) : 11
    XAUUSD,
    XAGUSD,
    XAUEUR,
    XAGEUR,
    XPDUSD,
    XPTUSD,
    XAUAUD,
    XAUCHF,
    XAUGBP,
    XAGAUD,
    XAUJPY,

    // Stocks USA
    Alibaba_Group,
    Microsoft,
    Amazon_com,
    Apple,
    Facebook,
    Alphabet_C,
    Netflix,
    Coinbase_Global,
    NVIDIA,
    Tesla,
    AT_T,
    Baidu,
    Citigroup,
    Goldman_Sachs_Group,
    IBM,
    JPMorgan_Chase,
    Mastercard,
    McDonalds,
    NIKE,
    Oracle,
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

    // 10_000. for USDEUR
    pub fn get_pip_multi(&self) -> f64 {
        let sym = self.get_conf();
        10_u32.pow(sym.pip as u32) as f64
    }

    pub fn cal_price(&self, price: f64, pip: f64) -> f64 {
        // let sym = self.get_conf();
        // let mutl = 10_u32.pow(sym.pip as u32) as f64;
        // let adder = 1. + (pip / mutl);
        // let p = price * adder;
        // self.rond(p)
        let p = 1. + (pip / 10_000.);
        self.rond(price * p)
    }

    // Round price to it's supported fraction
    pub fn rond(&self, price: f64) -> f64 {
        let sym = self.get_conf();
        let frac = 10_u32.pow(sym.digits as u32) as f64;
        ((price * frac) as u64) as f64 / frac
    }

    pub fn to_string(&self) -> String {
        let stocks = self.to_stocks_string();
        match stocks {
            None => {
                format!("{:?}", self)
            }
            Some(sr) => sr.to_string(),
        }
    }

    fn to_stocks_string(&self) -> Option<&str> {
        use Self::*;
        let r = match self {
            Alibaba_Group => Some("Alibaba_Group"),
            _ => None,
        };
        r
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
