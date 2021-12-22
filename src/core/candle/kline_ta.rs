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
    // pub sma100: f64,
    // pub ma3: f64,
    // pub hull: f64,
    pub mom: f64,
    pub roc: f64,
    pub atr: f64,
    pub rsi: f64,
    pub cci: f64,
    // pub vel: VelRes,
    pub macd: MACDOutput,
    pub fisher: FisherRes,
    // New trending
    pub vel1: VelRes,
    pub vel2: VelRes,
    // pub ma1: f64,
    // pub ma2: f64,
    // pub ma3: f64,
    // pub t_ema1: f64,
    // pub t_ema2: f64,
    // pub t_ema3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAMethods {
    // pub sma100: ta::SMA,
    // pub ema200: ta::EMA,
    // pub hull: ta::HMA,
    pub mom: ta::Momentum,
    pub roc: ta::ROC,
    pub atr: ta::ATR,
    pub rsi: ta::RSI,
    pub cci: ta::CCI,
    // pub vel: ta::Vel,
    pub macd: ta::MACD,
    pub fisher: ta::Fisher,
    // For trending
    pub vel1: ta::Vel,
    pub vel2: ta::Vel,
    // pub ma1: HMA,
    // pub ma2: HMA,
    // pub ma3: EMA,
    // pub t_ema1: EMA,
    // pub t_ema2: EMA,
    // pub t_ema3: EMA,
}

impl TAMethods {
    pub fn new(cfg: &CandleConfig) -> Self {
        TAMethods {
            // sma100: ta::SMA::new(100).unwrap(),
            // ema200: ta::EMA::new(20).unwrap(),
            // hull: ta::HMA::new(10).unwrap(),
            mom: ta::Momentum::new(14).unwrap(),
            roc: ta::ROC::new(10).unwrap(),
            atr: ta::ATR::new(14).unwrap(),
            rsi: ta::RSI::new(14).unwrap(),
            cci: ta::CCI::new(14).unwrap(),
            // vel: ta::Vel::new(cfg.vel_period as usize).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            // macd: ta::MACD::new(18, 34, 9).unwrap(),
            fisher: ta::Fisher::new(9, 6).unwrap(),
            // For trending
            vel1: ta::Vel::new(cfg.vel_period as usize).unwrap(),
            // vel2: ta::Vel::new(4 * cfg.vel_period as usize).unwrap(), // todo clean
            vel2: ta::Vel::new(20).unwrap(), // todo clean

                                                                      // ma1: HMA::new(3).unwrap(),
                                                                      // ma2: HMA::new(15).unwrap(),
                                                                      // ma3: EMA::new(20).unwrap(),
                                                                      // t_ema1: EMA::new(5).unwrap(),
                                                                      // t_ema2: EMA::new(15).unwrap(),
                                                                      // t_ema3: EMA::new(50).unwrap(),
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
            // sma100: tam.sma100.next(price),
            // ma3: tam.ema200.next(price),
            // hull: tam.hull.next(price),
            mom: tam.mom.next(price),
            roc: tam.roc.next(price),
            atr: tam.atr.next(&kl),
            rsi: tam.rsi.next(kl.close),
            cci: tam.cci.next(&kl),
            // vel: tam.vel.next_ohlc(&kl),
            macd: tam.macd.next(kl.close),
            fisher: tam.fisher.next(&kl),
            // For trending
            vel1: tam.vel1.next(kl.hlc3()),
            vel2: tam.vel2.next(kl.hlc3()),
            // ma1: tam.ma1.next(kl.hlc3()),
            // ma2: tam.ma2.next(kl.hlc3()),
            // ma3: tam.ma3.next(kl.hlc3()),
            // t_ema1: tam.t_ema1.next(kl.hlc3()),
            // t_ema2: tam.t_ema2.next(kl.hlc3()),
            // t_ema3: tam.t_ema3.next(kl.hlc3()),
        },
    };
    kta
}
