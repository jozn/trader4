use crate::bar::*;
use crate::bar::{Bar, PrimaryHolder};
use crate::ta::zigzag::ZigZagRes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TimeFrameJson {
    pub ohlc: Vec<JsonRowOHLC>,

    pub high_line: Vec<RowJson>,
    pub low_line: Vec<RowJson>,
    pub markers: Vec<MarkerJson>,

    pub ma1: Vec<RowJson>,
    pub ma_mom: Vec<RowJson>,

    // Velocity
    pub vel_avg: Vec<RowJson>,
    pub vel_end: Vec<RowJson>,

    // VelMom -- vm: vel_mom
    pub vm_mom: Vec<RowJson>,
    pub vm_sum: Vec<RowJson>,
    pub vm_count: Vec<RowJson>,

    pub bull_line: Vec<RowJson>,
    pub bear_line: Vec<RowJson>,

    // RPI
    pub rpi_high: Vec<RowJson>,
    pub rpi_low: Vec<RowJson>,

    // Dmi
    pub dmi_plus: Vec<RowJson>,
    pub dmi_minus: Vec<RowJson>,
    pub dmi_diff: Vec<RowJson>,

    // MACD
    pub macd_macd: Vec<RowJson>,
    pub macd_signal: Vec<RowJson>,
    pub macd_histogram: Vec<RowJson>,

    // DCSnake
    pub dcs_high: Vec<RowJson>,
    pub dcs_low: Vec<RowJson>,
    pub dcs_oversold: Vec<RowJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SkyJsonOut {
    pub major: TimeFrameJson,
    pub medium: TimeFrameJson,
    pub small: TimeFrameJson,

    pub markers: Vec<MarkerJson>,
    pub wave1: Vec<RowJson>,
    pub wave2: Vec<RowJson>,
    pub wave3: Vec<RowJson>,

    pub zigzag: Vec<RowJson>,
    pub zigzag2: Vec<ZigZagRes>,

    pub score_bull: Vec<RowJson>,
    pub score_bear: Vec<RowJson>,
    pub score_diff: Vec<RowJson>,

    pub major_ma_mom: Vec<RowJson>,

    // RDC - Relative DC
    pub rdc_med: Vec<RowJson>, // perc_med
    pub rdc_big: Vec<RowJson>,
    pub rdc_med_height: Vec<RowJson>,
    pub rdc_big_height: Vec<RowJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonRowOHLC {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl JsonRowOHLC {
    fn new(b: &Bar) -> JsonRowOHLC {
        Self {
            time: b.open_time / 1000,
            open: b.open,
            high: b.high,
            low: b.low,
            close: b.close,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RowJson {
    pub time: i64,
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MarkerJson {
    pub time: i64,
    pub position: String,
    pub color: String,
    pub shape: String,
    pub text: String,
}

pub fn bars_to_json_old(bars: Vec<PrimaryHolder>) -> TimeFrameJson {
    bars_to_json(&bars)
}
pub fn bars_to_json(bars: &Vec<PrimaryHolder>) -> TimeFrameJson {
    let mut out = TimeFrameJson::default();
    for ph in bars {
        out.ohlc.push(JsonRowOHLC::new(&ph.primary));

        let bar = &ph.primary;
        let ta = &ph.primary.ta;
        let pta = &ph.primary.ta;
        let bta = &ph.big.ta;
        let time = bar.open_time / 1000;

        /*// Set high/low lines
        out.high_line.push(RowJson {
            time: time,
            value: ta.rpi.high,
        });
        out.low_line.push(RowJson {
            time: time,
            value: ta.rpi.low,
        });*/

        // MA1
        out.ma1.push(RowJson {
            time,
            value: bta.ma1, // green
        });

        // MA Mom
        out.ma_mom.push(RowJson {
            time,
            value: bta.ma_mom,
        });

        // Velocity
        let vel = &bta.vel;
        out.vel_avg.push(RowJson {
            time,
            // value: vel.avg_vel_pip,
            value: vel.count as f64,
        });
        out.vel_end.push(RowJson {
            time,
            value: vel.end_vel_pip,
        });

        // VelMom
        let vm = &bta.vel_mom;
        out.vm_mom.push(RowJson {
            time,
            value: vm.ma_mom,
        });
        out.vm_sum.push(RowJson {
            time,
            value: vm.ma_sum,
        });
        out.vm_count.push(RowJson {
            time,
            value: vm.count as f64,
        });

        // Trend line
        out.bull_line.push(RowJson {
            time,
            value: bta.trend.bull_line, // green
        });
        out.bear_line.push(RowJson {
            time,
            value: bta.trend.bear_line,
        });

        // RPI
        // out.rpi_high.push(RowJson {
        //     time,
        //     value: pta.rpi.high,
        // });
        // out.rpi_low.push(RowJson {
        //     time,
        //     value: pta.rpi.low,
        // });

        // // Bollinger Bands
        // out.rpi_high.push(RowJson {
        //     time: time,
        //     value: ta.bb.high_band,
        // });
        // out.rpi_low.push(RowJson {
        //     time: time,
        //     value: ta.bb.low_band,
        // });

        // Gorilla Bands
        out.rpi_high.push(RowJson {
            time: time,
            value: ta.sb.high_band,
        });
        out.rpi_low.push(RowJson {
            time: time,
            value: ta.sb.low_band,
        });

        // DMI
        out.dmi_plus.push(RowJson {
            time,
            value: bta.dmi.plus, // green
        });
        out.dmi_minus.push(RowJson {
            time,
            value: bta.dmi.minus,
        });
        out.dmi_diff.push(RowJson {
            time,
            value: bta.dmi.adx,
        });

        // MACD
        out.macd_macd.push(RowJson {
            time,
            value: pta.macd.macd,
        });
        out.macd_signal.push(RowJson {
            time,
            value: pta.macd.signal,
        });
        out.macd_histogram.push(RowJson {
            time,
            value: pta.macd.histogram,
        });

        // DCSnake
        out.dcs_high.push(RowJson {
            time,
            value: pta.dc_snake.x_high,
        });
        out.dcs_low.push(RowJson {
            time,
            value: pta.dc_snake.x_low,
        });
        out.dcs_oversold.push(RowJson {
            time,
            value: pta.dc_snake.oversold_line,
        });
    }
    out
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TrendAnalyseOut {
    pub major: TimeFrameJson,
    pub tt: Vec<f64>,
}

// This trait must be implemented in sky_engs to extract jsons
pub trait JsonMaker {
    fn get_bars(&self) -> MultiBars;
    fn get_markers(&self, start: i64, end: i64) -> Vec<MarkerJson>;
    // Last chanse to set engine specifec data in jsons
    fn set_json_data(&self, jo: &mut SkyJsonOut);
}
