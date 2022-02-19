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

    // MACD
    pub macd_macd: Vec<RowJson>,
    pub macd_signal: Vec<RowJson>,
    pub macd_histogram: Vec<RowJson>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SkyJsonOut {
    pub major: TimeFrameJson,
    pub medium: TimeFrameJson,
    pub small: TimeFrameJson,

    pub markers: Vec<MarkerJson>,

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
            if !(bar.open_time >= start && bar.open_time <= end) {
                continue;
            }
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

            out.major_ma_mom.push(RowJson {
                time,
                value: fm.bar_major.big.ta.ma_mom,
            });

            // todo migrate markers from x_**
            // Markers
            if fm.buy2 {
                for s in fm.buys.clone() {
                    out.markers.push(MarkerJson {
                        time: s / 1000,
                        position: "belowBar".to_string(),
                        color: "#2196F3".to_string(),
                        shape: "arrowUp".to_string(),
                        text: format!(""), // text: format!("Sell @")
                                           // text: format!("Sell @ {}", bar.hlc3())
                    })
                }
            }
            if fm.sell2 {
                for s in fm.sells.clone() {
                    out.markers.push(MarkerJson {
                        time: s / 1000,
                        position: "aboveBar".to_string(),
                        color: "#e91e63".to_string(),
                        shape: "arrowDown".to_string(),
                        text: format!(""), // text: format!("Sell @")
                                           // text: format!("Sell @ {}", bar.hlc3())
                    })
                }
            }
        }

        // Sort markets asending
        out.markers.sort_by(|o1, o2| o1.time.cmp(&o2.time));

        out
    }
}
