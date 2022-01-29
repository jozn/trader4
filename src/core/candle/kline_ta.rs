use super::*;
use crate::base::OHLCV;
use crate::ta;
use crate::ta::*;
use crate::ta::{FisherRes, MACDOutput_Dep};
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
    pub macd: MACDOutput_Dep,
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
    pub macd_dep: ta::MACDDep,
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
            macd_dep: ta::MACDDep::new(12, 26, 9).unwrap(),
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

    let mut kta = KlineTA {
        kline: kline.clone(),
        is_completed: false,
        ta1: TA1 {
            mom: tam.mom.next(price),
            roc: tam.roc.next(price),
            atr: tam.atr.next(&kl),
            rsi: tam.rsi.next(kl.close),
            cci: tam.cci.next(&kl),
            macd: tam.macd_dep.next(kl.close),
            fisher: tam.fisher.next(&kl),
            // For trending
            vel1: tam.vel1.next(kl.hlc3()),
            vel2: tam.vel2.next(kl.hlc3()),
            ta2: TA2 {
                atr: tam2.atr.next(&kl),
                macd: tam2.macd.next(kl.close),
                dmi: tam2.dmi.next(&kl),
                stoch: tam2.stoch.next(&kl),
                trend: tam2.trend.next(&kl),
                // roc_macd: tam2.roc_macd.next(price),
                dc: tam2.dc.next(&kl),
                dcs: tam2.dcs.next(&kl),
                vel1: tam2.vel1.next(kl.hlc3()),
                vel2: tam2.vel2.next(kl.hlc3()),
                rsi: tam2.rsi.next(kl.hlc3()),
                rsi_sth: tam2.rsi_stoch.next(kl.hlc3()),
                ..Default::default()
            },
            // ..Default::default() // All comment above indicarors
        },
    };
    let ta2 = kta.ta1.ta2.clone();
    kta.ta1.ta2.roc_macd = tam2.roc_macd.next(ta2.macd.macd);
    kta.ta1.ta2.roc_dmi_up = tam2.roc_dmi_up.next(ta2.dmi.plus);
    kta.ta1.ta2.roc_dmi_down = tam2.roc_dmi_down.next(ta2.dmi.minus);
    kta.ta1.ta2.roc_stoch = tam2.roc_stoch.next(ta2.stoch.main_k);

    kta
}

///////////// Version 2 of TA2 - Donchain Channel ///////////////

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TA2 {
    pub atr: f64,
    pub macd: MACDOutput,
    pub dmi: DMIOutput,
    pub stoch: StochRes,
    pub trend: MATrendOut,
    pub roc_macd: f64,
    pub roc_dmi_up: f64,
    pub roc_dmi_down: f64,
    pub roc_stoch: f64,

    pub dc: DCRes,
    pub dcs: DCSRes,
    pub vel1: VelRes,
    pub vel2: VelRes2,
    pub rsi: f64,
    pub rsi_sth: StochRes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TA2Methods {
    pub atr: ta::ATR,
    pub macd: ta::MACD,
    pub dmi: ta::DMI,
    pub stoch: ta::Stoch,
    pub trend: ta::MATrend,
    pub roc_macd: ROC,
    pub roc_dmi_up: ROC,
    pub roc_dmi_down: ROC,
    pub roc_stoch: ROC,

    pub dc: ta::DC,
    pub dcs: ta::DCS,
    pub vel1: ta::Vel,
    pub vel2: ta::Vel2,
    pub rsi: ta::RSI,
    pub rsi_stoch: ta::StochRSI,
}

impl TA2Methods {
    pub fn new(cfg: &CandleConfig) -> Self {
        Self {
            atr: ta::ATR::new(14).unwrap(),
            macd: ta::MACD::new(12, 26, 9).unwrap(),
            dmi: ta::DMI::new(14, 14).unwrap(),
            stoch: ta::Stoch::new(14, 3, 5).unwrap(),
            trend: ta::MATrend::new(10).unwrap(),
            roc_macd: ta::ROC::new(3).unwrap(),
            roc_dmi_up: ta::ROC::new(3).unwrap(),
            roc_dmi_down: ta::ROC::new(3).unwrap(),
            roc_stoch: ta::ROC::new(3).unwrap(),

            dc: ta::DC::new(20).unwrap(),
            // dcs: ta::DCS::new(80).unwrap(),
            dcs: ta::DCS::new(50).unwrap(),
            vel1: ta::Vel::new(cfg.vel1_period as usize).unwrap(),
            vel2: ta::Vel2::new(cfg.vel2_period as usize).unwrap(),
            rsi: ta::RSI::new(14).unwrap(),
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
