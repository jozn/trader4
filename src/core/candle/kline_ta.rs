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
    pub mom: f64,
    pub roc: f64,
    pub atr: f64,
    pub rsi: f64,
    pub cci: f64,
    pub macd: MACDOutput,
    pub fisher: FisherRes,
    // New trending
    pub vel1: VelRes,
    pub vel2: VelRes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAMethods {
    pub mom: ta::Momentum,
    pub roc: ta::ROC,
    pub atr: ta::ATR,
    pub rsi: ta::RSI,
    pub cci: ta::CCI,
    pub macd: ta::MACD,
    pub fisher: ta::Fisher,
    // For trending
    pub vel1: ta::Vel,
    pub vel2: ta::Vel,
}

impl TAMethods {
    pub fn new(cfg: &CandleConfig) -> Self {
        TAMethods {
            mom: ta::Momentum::new(14).unwrap(),
            roc: ta::ROC::new(10).unwrap(),
            atr: ta::ATR::new(14).unwrap(),
            rsi: ta::RSI::new(14).unwrap(),
            cci: ta::CCI::new(14).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            fisher: ta::Fisher::new(9, 6).unwrap(),
            // For trending
            vel1: ta::Vel::new(cfg.vel1_period as usize).unwrap(),
            vel2: ta::Vel::new(cfg.vel2_period as usize).unwrap(),
        }
    }
}

// should not be used, just to satasfy compiler for Default
impl Default for TAMethods {
    fn default() -> Self {
        let cfg = CandleConfig::default();
        TAMethods::new(&cfg)
    }
}

pub fn cal_indicators(tam: &mut TAMethods, kline: &Kline) -> KlineTA {
    let kl = kline;

    let price = kl.hlc3();

    let kta = KlineTA {
        kline: kline.clone(),
        is_completed: false,
        ta1: TA1 {
            mom: tam.mom.next(price),
            roc: tam.roc.next(price),
            atr: tam.atr.next(&kl),
            rsi: tam.rsi.next(kl.close),
            cci: tam.cci.next(&kl),
            macd: tam.macd.next(kl.close),
            fisher: tam.fisher.next(&kl),
            // For trending
            vel1: tam.vel1.next(kl.hlc3()),
            vel2: tam.vel2.next(kl.hlc3()),
        },
    };
    kta
}
