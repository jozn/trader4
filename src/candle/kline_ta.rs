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
    pub vel: VelRes,
    pub macd: MACDOutput,
    pub fisher: FisherRes,
    // New trending
    pub t_hull1: f64,
    pub t_hull2: f64,
    pub t_hull3: f64,
    pub t_ema1: f64,
    pub t_ema2: f64,
    pub t_ema3: f64,
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
    pub vel: ta::Vel,
    pub macd: ta::MACD,
    pub fisher: ta::Fisher,
    // For trending
    pub t_hull1: HMA,
    pub t_hull2: HMA,
    pub t_hull3: HMA,
    pub t_ema1: EMA,
    pub t_ema2: EMA,
    pub t_ema3: EMA,
}

// should not be used, just to satasfy compiler for Default
impl Default for TAMethods {
    fn default() -> Self {
        TAMethods {
            sma50: ta::SMA::new(100).unwrap(),
            ema: ta::EMA::new(100).unwrap(),
            hull: ta::HMA::new(10).unwrap(),
            mom: ta::Momentum::new(10).unwrap(),
            roc: ta::ROC::new(10).unwrap(),
            rsi: ta::RSI::new(14).unwrap(),
            cci: ta::CCI::new(14).unwrap(),
            vel: ta::Vel::new(50).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            fisher: ta::Fisher::new(9, 6).unwrap(),
            // For trending
            t_hull1: HMA::new(3).unwrap(),
            t_hull2: HMA::new(15).unwrap(),
            t_hull3: HMA::new(55).unwrap(),
            t_ema1: EMA::new(5).unwrap(),
            t_ema2: EMA::new(15).unwrap(),
            t_ema3: EMA::new(50).unwrap(),
        }
    }
}

pub fn cal_indicators(tam: &mut TAMethods, kline: &Kline) -> KlineTA {
    let kl = kline;

    let price = kl.hlc3();

    let kta = KlineTA {
        kline: kline.clone(),
        is_completed: false,
        ta1: TA1 {
            sma50: tam.sma50.next(price),
            ema10: tam.ema.next(price),
            hull: tam.hull.next(price),
            mom: tam.mom.next(price),
            roc: tam.roc.next(price),
            rsi: tam.rsi.next(kl.close),
            cci: tam.cci.next(&kl),
            vel: tam.vel.next(&kl),
            macd: tam.macd.next(kl.close),
            fisher: tam.fisher.next(&kl),
            // For trending
            t_hull1: tam.t_hull1.next(kl.hlc3()),
            t_hull2: tam.t_hull2.next(kl.hlc3()),
            t_hull3: tam.t_hull3.next(kl.hlc3()),
            t_ema1: tam.t_ema1.next(kl.hlc3()),
            t_ema2: tam.t_ema2.next(kl.hlc3()),
            t_ema3: tam.t_ema3.next(kl.hlc3()),
        },
    };
    kta
}
