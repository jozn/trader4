use super::*;
use crate::base::*;
use crate::candle::{Kline, KlineTA, Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SFrame {
    pub fid: u64, // frame_id
    pub finished: bool,
    pub duration: String,

    // Donchain Channel
    pub med_low: f64,
    pub med_high: f64,
    #[serde(skip)]
    pub med_mid: f64,
    pub big_low: f64,
    pub big_high: f64,
    #[serde(skip)]
    pub big_mid: f64,

    pub spreed_min: f64,
    pub spreed_max: f64,
    pub med_dc_hl_pip: f64,
    pub big_dc_hl_pip: f64,

    #[serde(skip)]
    pub vel: VelRes,
    pub atr_p: f64,
    pub rsi: f64,
    #[serde(skip)]
    pub rsi_sth: StochRes, // rsi_stoch
    #[serde(skip)]
    pub vel2: VelRes2,

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: SCandle,
    #[serde(skip)]
    pub macd: MACDOutput,
    #[serde(skip)]
    pub dmi: DMIOutput,
    #[serde(skip)]
    pub stoch: StochRes,
    #[serde(skip)]
    pub trend: MATrendOut,

    // big
    #[serde(skip)]
    pub b_trend: MATrendOut,

    #[serde(skip)]
    pub score: Score,

    pub roc_macd: f64,
    pub roc_dmi_up: f64,
    pub roc_dmi_down: f64,
    pub roc_stoch: f64,
}

pub type FrameCsv = (
    SCandle,
    Score,
    MACDOutput,
    DMIOutput,
    StochRes,
    MATrendOut,
    SFrame,
    MATrendOut,
);

impl SFrame {
    pub fn to_csv(&self) -> FrameCsv {
        (
            self.ohlc.clone(),
            self.score.clone(),
            self.macd.clone(),
            self.dmi.clone(),
            self.stoch.clone(),
            self.trend.clone(),
            // "BIG".to_string(),
            self.clone(),
            self.b_trend.clone(),
        )
    }

    pub fn set_spread(&mut self, ticks: &TimeSerVec<Tick>) {
        // println!("se {}", ticks.len());
        self.spreed_min = f64::MAX;
        for t in ticks.get_vec() {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > self.spreed_max {
                self.spreed_max = spread;
            }
            if spread < self.spreed_min {
                self.spreed_min = spread;
            }
        }
    }

    pub fn set_trend(&mut self) {}
}

pub fn new_frame(k_med: &KlineTA, k_big: &KlineTA) -> SFrame {
    let med_k = &k_med.kline;
    let big_k = &k_big.kline;

    let med_ta = &k_med.ta1.ta2;
    let big_ta = &k_big.ta1.ta2;

    let dur = med_k.close_time - med_k.open_time;

    let mut frame = SFrame {
        fid: med_k.kid,
        finished: false,
        duration: helper::to_duration(dur as i64),
        med_low: med_ta.dc.low,
        med_high: med_ta.dc.high,
        med_mid: (med_ta.dc.high + med_ta.dc.low) / 2.,
        big_low: big_ta.dc.low,
        big_high: big_ta.dc.high,
        big_mid: (big_ta.dc.high + big_ta.dc.low) / 2.,
        spreed_min: 0.0,
        spreed_max: 0.0,
        med_dc_hl_pip: (med_ta.dc.high - med_ta.dc.low) * 10_000.,
        big_dc_hl_pip: (big_ta.dc.high - big_ta.dc.low) * 10_000.,
        vel: med_ta.vel1.clone(),
        atr_p: med_ta.atr * 10_000.,
        rsi: med_ta.rsi,
        rsi_sth: big_ta.rsi_sth.clone(),
        vel2: med_ta.vel2.clone(),
        ohlc: SCandle::new(med_k),

        macd: med_ta.macd.clone(),
        dmi: med_ta.dmi.clone(),
        stoch: med_ta.stoch.clone(),
        trend: med_ta.trend.clone(),

        //big
        b_trend: big_ta.trend.clone(),

        roc_macd: med_ta.roc_macd,
        roc_dmi_up: med_ta.roc_dmi_up,
        roc_dmi_down: med_ta.roc_dmi_down,
        roc_stoch: med_ta.roc_stoch,

        ..Default::default()
    };

    frame.score = Score::new(&frame);

    frame
}
