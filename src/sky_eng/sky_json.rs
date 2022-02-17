use super::*;

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

    // Dmi
    pub dmi_plus: Vec<RowJson>,
    pub dmi_minus: Vec<RowJson>,
    pub dmi_diff: Vec<RowJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SkyJsonOut {
    pub major: TimeFrameJson,
    pub medium: TimeFrameJson,
    pub small: TimeFrameJson,

    pub score_bull: Vec<RowJson>,
    pub score_bear: Vec<RowJson>,
    pub score_diff: Vec<RowJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SkyJsonOutDep {
    pub ohlc: Vec<JsonRowOHLC>,
    pub high_line: Vec<RowJson>,
    pub low_line: Vec<RowJson>,
    pub markers: Vec<MarkerJson>,

    pub bull_line: Vec<RowJson>,
    pub bear_line: Vec<RowJson>,

    pub score_bull: Vec<RowJson>,
    pub score_bear: Vec<RowJson>,
    pub score_diff: Vec<RowJson>,

    // samples
    pub sample_1: Vec<RowJson>,
    pub sample_2: Vec<RowJson>,
    pub sample_3: Vec<RowJson>,

    pub dmmd_1: Vec<RowJson>,
    pub dmmd_2: Vec<RowJson>,

    // Major timeline
    pub major_ohlc: Vec<JsonRowOHLC>,

    // Primary timeline
    pub medium_ohlc: Vec<JsonRowOHLC>,

    // Small timeline
    pub small_ohlc: Vec<JsonRowOHLC>,
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

        // Set high/low lines
        out.high_line.push(RowJson {
            time: time,
            value: ta.rpi.high,
        });
        out.low_line.push(RowJson {
            time: time,
            value: ta.rpi.low,
        });

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

        /*        out.dmi_plus.push(RowJson {
            time,
            value: bta.dmmd.ma_fast, // green
        });
        out.dmi_minus.push(RowJson {
            time,
            value: bta.dmmd.ma_slow,
        });
        out.dmi_diff.push(RowJson {
            time,
            value: bta.dmmd.color,
        });*/
    }
    out
}

fn bars_to_json_raw_dep(bars: Vec<Bar>) -> Vec<JsonRowOHLC> {
    let mut out = vec![];
    for b in bars {
        out.push(JsonRowOHLC::new(&b))
    }
    out
}

impl SkyEng {
    pub fn to_json(&self, start: i64, end: i64) -> SkyJsonOut {
        let s = self;
        let mut out = SkyJsonOut::default();
        out.major = bars_to_json(s.major_bars.get_bars_ph(start, end));
        out.medium = bars_to_json(s.medium_bars.get_bars_ph(start, end));
        out.small = bars_to_json(s.small_bars.get_bars_ph(start, end));

        for fm in s.frames.iter() {
            let bar = &fm.bar_medium.primary;
            let time = bar.open_time / 1000;

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
        }
        out
    }

    pub fn to_json_dep(&self, start: i64, end: i64) -> SkyJsonOutDep {
        let s = self;
        let mut out = SkyJsonOutDep::default();
        out.major_ohlc = bars_to_json_raw_dep(s.major_bars.get_primary_bars(start, end));
        out.medium_ohlc = bars_to_json_raw_dep(s.medium_bars.get_primary_bars(start, end));
        out.ohlc = bars_to_json_raw_dep(s.medium_bars.get_primary_bars(start, end));
        out.small_ohlc = bars_to_json_raw_dep(s.small_bars.get_primary_bars(start, end));

        for fm in s.frames.iter() {
            // out.ohlc.push(JsonRowOHLC::new(&fm.bar.primary));
            let b = &fm.bar_medium.primary;
            if !(b.open_time >= start && b.open_time <= end) {
                continue;
            }
            // Set high/low lines
            let bar = &fm.bar_medium.primary;
            let ta = &bar.ta;
            let pta = &fm.bar_medium.primary.ta;
            let bta = &fm.bar_medium.big.ta;
            let time = bar.open_time / 1000;
            out.high_line.push(RowJson {
                time: bar.open_time / 1000,
                value: ta.rpi.high,
            });
            out.low_line.push(RowJson {
                time: bar.open_time / 1000,
                value: ta.rpi.low,
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

            // Set buy/sell markers
            if fm.buy1 {
                out.markers.push(MarkerJson {
                    time,
                    position: "belowBar".to_string(),
                    color: "#2196F3".to_string(),
                    shape: "arrowUp".to_string(),
                    text: format!(""), // text: format!("Buy @")
                                       // text: format!("Buy @ {}", bar.hlc3())
                })
            }
            if fm.sell1 {
                out.markers.push(MarkerJson {
                    time,
                    position: "aboveBar".to_string(),
                    color: "#e91e63".to_string(),
                    shape: "arrowDown".to_string(),
                    text: format!(""), // text: format!("Sell @")
                                       // text: format!("Sell @ {}", bar.hlc3())
                })
            }

            // Add scores
            let score = &fm.score;
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

            // Add sample
            let dmi = &pta.dmi;
            // let dmi = &bta.dmi;
            out.sample_1.push(RowJson {
                time,
                value: dmi.plus as f64,
            });
            out.sample_2.push(RowJson {
                time,
                value: dmi.minus as f64,
            });
            out.sample_3.push(RowJson {
                time,
                value: dmi.dmx,
                // value: dmi.adx,
            });

            // DMMD
            let dmmd = &bta.dmmd;
            // let dmmd = &pta.dmmd;
            out.dmmd_1.push(RowJson {
                time,
                // value: dmmd.ma_fast,
                value: dmmd.diff,
            });
            out.dmmd_2.push(RowJson {
                time,
                // value: dmmd.ma_slow,
                value: dmmd.color,
            });

            // Sample 2 - DMMD
            // let dmmd = &bta.dmmd;
            // out.sample_1.push(RowJson {
            //     time,
            //     value: dmmd.ma_fast as f64,
            // });
            // out.sample_2.push(RowJson {
            //     time,
            //     value: dmmd.ma_slow as f64,
            // });
            // out.sample_3.push(RowJson {
            //     time,
            //     value: dmmd.histogram,
            // });
        }
        out
    }
}
