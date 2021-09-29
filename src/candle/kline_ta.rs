use super::*;
use crate::base::OHLCV;
use crate::ta;
use crate::ta::*;
use crate::ta::{FisherRes, MACDOutput};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KlineTA {
    pub kline: Kline,
    pub is_completed: bool, // todo remove
    pub ta1: TA1,
}

impl KlineTA {}

impl KlineId for KlineTA {
    fn get_kline_id(&self) -> u64 {
        self.kline.bucket
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TA1 {
    pub sma50: f64,
    pub ema10: f64,
    pub hull: f64,
    pub mom: f64,
    pub roc: f64,
    pub rsi: f64,
    pub cci: f64,
    pub macd: MACDOutput,
    pub fisher: FisherRes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAMethods {
    pub sma50: ta::SMA,
    pub ema: ta::EMA,
    pub hull: ta::HMA,
    pub mom: ta::Momentum,
    pub roc: ta::ROC,
    pub rsi: ta::RSI,
    pub cci: ta::CCI,
    pub macd: ta::MACD,
    pub fisher: ta::Fisher,
}

// should not be used, just to satasfy compiler for Default
impl Default for TAMethods {
    fn default() -> Self {
        TAMethods {
            sma50: ta::SMA::new(100).unwrap(),
            ema: ta::EMA::new(14).unwrap(),
            hull: ta::HMA::new(10).unwrap(),
            mom: ta::Momentum::new(10).unwrap(),
            roc: ta::ROC::new(10).unwrap(),
            rsi: ta::RSI::new(14).unwrap(),
            cci: ta::CCI::new(14).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            fisher: ta::Fisher::new(9, 6).unwrap(),
        }
    }
}

pub fn cal_indicators(tam: &mut TAMethods, kline: &Kline) -> KlineTA {
    let r = kline;

    let val = r.hlc3();

    let kta = KlineTA {
        kline: kline.clone(),
        is_completed: false,
        ta1: TA1 {
            sma50: tam.sma50.next(val),
            ema10: tam.ema.next(val),
            hull: tam.hull.next(val),
            mom: tam.mom.next(val),
            roc: tam.roc.next(val),
            rsi: tam.rsi.next(r.close),
            cci: tam.cci.next(&r),
            macd: tam.macd.next(r.close),
            fisher: tam.fisher.next(&r),
        },
    };
    kta
}
