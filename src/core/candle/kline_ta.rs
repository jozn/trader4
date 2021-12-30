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
    pub ta2: TA2, // We insert it here in order to avoid changing Candles source codes
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
    pub ta2: TA2Methods,
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
            ta2: TA2Methods::new(cfg),
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
    let mut tam2 = &mut tam.ta2;

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
            ta2: TA2 {
                atr: tam2.atr.next(&kl),
                dc: tam2.dc.next(&kl),
                vel1: tam2.vel1.next(kl.hlc3()),
                vel2: tam2.vel2.next(kl.hlc3()),
                rsi: tam2.rsi.next(kl.hlc3()),
                rsi_sth: tam2.rsi_stoch.next(kl.hlc3()),
            },
            // ..Default::default() // All comment above indicarors
        },
    };
    kta
}

///////////// Version 2 of TA2 - Donchain Channel ///////////////

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TA2 {
    pub atr: f64,
    pub dc: DCRes,
    pub vel1: VelRes,
    pub vel2: VelRes2,
    pub rsi: f64,
    pub rsi_sth: StochRes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TA2Methods {
    pub atr: ta::ATR,
    pub dc: ta::DC,
    pub vel1: ta::Vel,
    pub vel2: ta::Vel2,
    pub rsi: ta::RSI,
    pub rsi_stoch: ta::StochRSI,
}

impl TA2Methods {
    pub fn new(cfg: &CandleConfig) -> Self {
        Self {
            atr: ta::ATR::new(14).unwrap(),
            dc: ta::DC::new(20).unwrap(),
            vel1: ta::Vel::new(cfg.vel1_period as usize).unwrap(),
            vel2: ta::Vel2::new(cfg.vel2_period as usize).unwrap(),
            rsi: ta::RSI::new(cfg.vel2_period as usize).unwrap(),
            rsi_stoch: ta::StochRSI::new(14, 1, 3).unwrap(),
        }
    }
}

// should not be used, just to satasfy compiler for Default
impl Default for TA2Methods {
    fn default() -> Self {
        let cfg = CandleConfig::default();
        TA2Methods::new(&cfg)
    }
}
