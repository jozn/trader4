use super::*;
use crate::bar::*;
use crate::offline::Position;
use crate::ta::zigzag::{ZigZag, ZigZagRes};
use crate::ta::Wave;
use crate::{analyse, offline};
use std::os::unix::raw::off_t;

// todo: extract json to core

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TimeFrameJson {
    pub ohlc: Vec<JsonRowOHLC>,

    pub high_line: Vec<RowJson>,
    pub low_line: Vec<RowJson>,
    pub markers: Vec<MarkerJson>,

    pub ma1: Vec<RowJson>,
    pub ma_mom: Vec<RowJson>,

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

fn bars_to_json(bars: Vec<PrimaryHolder>) -> TimeFrameJson {
    let mut out = TimeFrameJson::default();
    for ph in &bars {
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

impl SkyEng {
    pub fn to_json(&self, start: i64, end: i64, pos: &Vec<Position>) -> SkyJsonOut {
        let s = self;
        let mut out = SkyJsonOut::default();
        out.major = bars_to_json(s.major_bars.get_bars_ph(start, end));
        out.medium = bars_to_json(s.medium_bars.get_bars_ph(start, end));
        out.small = bars_to_json(s.small_bars.get_bars_ph(start, end));

        let mut zigzag = ZigZag::default();
        let mut wave1 = Wave::new(14, 7, 0.05).unwrap();
        let mut wave2 = Wave::new(14, 7, 0.10).unwrap();
        let mut wave3 = Wave::new(14, 7, 0.20).unwrap();

        for fm in &s.frames {
            let bar = &fm.bar_medium.primary;
            if !(bar.open_time >= start && bar.open_time <= end) {
                continue;
            }
            let time = bar.open_time / 1000;
            wave1.next(bar);
            wave2.next(bar);
            wave3.next(bar);
            // zigzag
            let zigr = zigzag.next(bar);
            match zigr {
                None => {}
                Some(z) => {
                    out.zigzag2.push(z.clone());
                    // out.zigzag.push(RowJson {
                    //     time: z.time/1000,
                    //     value: z.price,
                    // });
                }
            }

            // Add scores
            let score = &fm.tscore;
            out.score_bull.push(RowJson {
                time,
                value: score.bull as f64,
            });
            out.score_bear.push(RowJson {
                time,
                value: -score.bear as f64,
            });
            out.score_diff.push(RowJson {
                time,
                value: score.diff as f64,
            });

            out.major_ma_mom.push(RowJson {
                time,
                value: fm.bar_major.big.ta.ma_mom,
            });

            // todo migrate markers from old frame
            // Markers
            if fm.get_early_mark().is_some() {
                out.markers.push(fm.get_early_mark().unwrap());
            }
            if fm.get_long_final_mark().is_some() {
                out.markers.push(fm.get_long_final_mark().unwrap());
            }
        }

        // for z in &zigzag.store {
        // for z in &wave1.wave_ress {
        //     out.zigzag.push(RowJson {
        //         time: z.time / 1000,
        //         value: z.price,
        //     });
        // }

        // Waves
        for z in &wave1.wave_ress {
            out.wave1.push(RowJson {
                time: z.time / 1000,
                value: z.price,
            });
        }
        for z in &wave2.wave_ress {
            out.wave2.push(RowJson {
                time: z.time / 1000,
                value: z.price,
            });
        }
        for z in &wave3.wave_ress {
            out.wave3.push(RowJson {
                time: z.time / 1000,
                value: z.price,
            });
        }

        // Motion Analyse
        use crate::core::analyse::wave_motion;
        // let mots = analyse::gen_motion(&wave3.wave_ress);
        let mo_gen =
            wave_motion::MotionGen::new(&wave3.wave_ress, &wave2.wave_ress, &wave1.wave_ress);
        // let mo_gen = analyse::MotionGen::new(&wave3.wave_ress,&wave1.wave_ress,&vec![]);
        let mots = mo_gen.run();
        println!("mots: {:#?}", mots);

        // Add trades(postions) to markers
        let trade_markers = offline::position_html::to_json_marker(&pos);
        for tm in trade_markers {
            out.markers.push(tm);
        }
        // Sort markets asending
        out.markers.sort_by(|o1, o2| o1.time.cmp(&o2.time));
        // out.markers.clear();
        out
    }
}
