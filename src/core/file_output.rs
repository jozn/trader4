use super::brain::*;
use crate::bar::{MultiBars, PrimaryHolder};
use crate::configs::assets::Pair;
use crate::helper::to_csv_out_v2;
use crate::json_output::{
    bars_to_json, JsonMaker, MarkerJson, RowJson, SkyJsonOut, TrendAnalyseOut,
};
use crate::offline;
use crate::offline::{Money, Position};
use crate::sig_engs::ml_eng::MLEng;
use crate::sig_engs::*;
use crate::ta::zigzag::ZigZag;
use crate::ta::Wave;
use crate::types::{DayInfo, WeekInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const OUT_FOLDER: &'static str = "/mnt/t/trader_out/v17/data_sky_web/";
const OUT_FOLDER_TREND: &'static str = "/mnt/t/trader_out/v17/trend/";

#[derive(Clone, Debug, Default)]
pub struct FilesOutputConfig {
    pub week_data: Vec<WeekInfo>,
    pub pair: Pair,
    pub print: bool,
    pub report: bool,
    pub days_out: bool,
    pub web: bool,
}

pub struct FilesOutput {
    pub cfg: FilesOutputConfig,
    pub week_data: Vec<WeekInfo>,
}

impl FilesOutput {
    pub fn run_sig_eng(
        &mut self,
        postions: &Vec<Position>,
        pair_mem: &PairSigHolder,
        money: &Money,
    ) {
        println!("web {:?} ...", &pair_mem.pair);
        self.write_web_output_one_eng(&pair_mem.ml_eng, &postions);

        if self.cfg.print {
            println!("{:#?}", money.balance);
        }

        // todo - get report by date range
        if self.cfg.report {
            // back_ref.report_to_folder(&self.week_data, &self.pair);
        }
    }
}

impl FilesOutput {
    // code copy of trans_wky_web3.rs
    fn write_web_output_one_eng(&self, json_maker: &impl JsonMaker, pos: &Vec<Position>) {
        let _bars = json_maker.get_bars();
        let pair = &self.cfg.pair;
        for wd in &self.week_data {
            let poss = get_postions_range(&pos, wd.start_ms, wd.end_ms);
            let mut sfg = SingleFileGen {
                cfg: self.cfg.clone(),
                week: wd.clone(),
                day: None,
                week_id: wd.week_id,
                day_num: 0,
                start: wd.start_ms,
                end: wd.end_ms,
                pos: vec![],
                markers: vec![],
                major_bars: vec![],
                medium_bars: vec![],
                small_bars: vec![],
            };
            sfg.set_data(json_maker, pos, wd.start_ms, wd.end_ms);
            sfg.write_json(json_maker);

            // Daily
            let mut start = wd.start_ms;
            let mut end = start + 86_400_000;
            let mut day_num = 1;
            if self.cfg.days_out {
                'days: loop {
                    if day_num == 8 {
                        break 'days;
                    }
                    let mut sfg = SingleFileGen {
                        cfg: self.cfg.clone(),
                        week: wd.clone(),
                        day: None,
                        week_id: wd.week_id,
                        day_num: day_num,
                        start,
                        end,
                        pos: vec![],
                        markers: vec![],
                        major_bars: vec![],
                        medium_bars: vec![],
                        small_bars: vec![],
                    };
                    sfg.set_data(json_maker, pos, start, end);

                    sfg.write_json(json_maker);

                    start = end;
                    end = start + 86_400_000;
                    day_num += 1;
                    // break; // todo remove
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SingleFileGen {
    pub cfg: FilesOutputConfig,
    pub week: WeekInfo,
    pub day: Option<DayInfo>, // manual is preferd for sunday workaround
    pub week_id: i32,
    pub day_num: i32,
    pub start: i64,
    pub end: i64,
    pub pos: Vec<Position>,
    pub markers: Vec<MarkerJson>,
    pub major_bars: Vec<PrimaryHolder>,
    pub medium_bars: Vec<PrimaryHolder>,
    pub small_bars: Vec<PrimaryHolder>,
}

impl SingleFileGen {
    // Common for week and days
    fn set_data(
        &mut self,
        // bars: &MultiBars,
        json_maker: &impl JsonMaker,
        pos: &Vec<Position>,
        time_start: i64,
        time_end: i64,
    ) {
        let pair = &self.cfg.pair;
        let bars = json_maker.get_bars();
        self.pos = get_postions_range(&pos, time_start, time_end);
        self.markers = json_maker.get_markers(time_start, time_end);
        self.major_bars = bars.major_bars.get_bars_ph(time_start, time_end);
        self.medium_bars = bars.medium_bars.get_bars_ph(time_start, time_end);
        self.small_bars = bars.small_bars.get_bars_ph(time_start, time_end);
    }
    fn to_json(&self) -> SkyJsonOut {
        let s = self;
        let mut out = SkyJsonOut::default();
        out.major = bars_to_json(&s.major_bars);
        out.medium = bars_to_json(&s.medium_bars);
        out.small = bars_to_json(&s.small_bars);

        let mut zigzag = ZigZag::default();
        let mut wave1 = Wave::new(14, 7, 0.05).unwrap();
        let mut wave2 = Wave::new(14, 7, 0.10).unwrap();
        let mut wave3 = Wave::new(14, 7, 0.20).unwrap();

        for fm in &s.medium_bars {
            let bar = &fm.primary;
            if !(bar.open_time >= self.start && bar.open_time <= self.end) {
                continue;
            }
            let time = bar.open_time / 1000;

            // Spread
            out.spread_max.push(RowJson {
                time,
                value: bar.spreed_max,
            });
            out.spread_min.push(RowJson {
                time,
                value: bar.spreed_min,
            });

            // Relative DC
            let rdc_ta = &bar.ta.rdc;
            out.rdc_med.push(RowJson {
                time,
                value: rdc_ta.perc_med,
            });
            out.rdc_big.push(RowJson {
                time,
                value: rdc_ta.perc_big,
            });
            out.rdc_med_height.push(RowJson {
                time,
                value: rdc_ta.height_med,
            });
            out.rdc_big_height.push(RowJson {
                time,
                value: rdc_ta.height_big,
            });

            // Relative Price (rp)
            let rdc_ta = &bar.ta.rel_price_dep;
            out.rp_os_med.push(RowJson {
                time,
                value: rdc_ta.oversold_med,
            });
            out.rp_os_big.push(RowJson {
                time,
                value: rdc_ta.oversold_big,
            });
            out.rp_osi_med.push(RowJson {
                time,
                value: rdc_ta.osi_med,
            });
            out.rp_osi_big.push(RowJson {
                time,
                value: rdc_ta.osi_big,
            });
            out.rp_stoch_main.push(RowJson {
                time,
                value: rdc_ta.os_stoch_main,
            });
            out.rp_stoch_smooth.push(RowJson {
                time,
                value: rdc_ta.os_stoch_smooth,
            });
            out.rp_med_height.push(RowJson {
                time,
                value: rdc_ta.height_med,
            });
            out.rp_big_height.push(RowJson {
                time,
                value: rdc_ta.height_big,
            });

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

            out.major_ma_mom.push(RowJson {
                time,
                value: fm.big.ta.ma_mom,
            });

        }

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

        //////////// Motion Analyse
        use crate::core::analyse::wave_motion;
        let mo_gen =
            wave_motion::MotionGen::new(&wave3.wave_ress, &wave2.wave_ress, &wave1.wave_ress);
        let mots = mo_gen.run();
        // println!("mots: {:#?}", mots);
        ///////////

        /////////// Markers ////////////
        // Add sig_engs markers
        out.markers = self.markers.clone();
        // Add trades(postions) to markers
        let trade_markers = offline::position_html::to_json_marker(&self.pos);
        for tm in trade_markers {
            out.markers.push(tm);
        }
        // Sort markets asending
        out.markers.sort_by(|o1, o2| o1.time.cmp(&o2.time));
        println!("market lern: {:?}", out.markers.len());
        // Medium markers unique
        let cp_markers = out.markers.clone();
        let mut mpm = HashMap::new();
        cp_markers.iter().for_each(|m| {
            mpm.insert(&m.marker_key, m.clone());
        });
        for (_, m) in mpm {
            out.markers_med.push(m);
        }
        out.markers_med.sort_by(|o1, o2| o1.time.cmp(&o2.time));
        // out.markers.clear();
        out
    }

    fn write_json(&self, json_maker: &impl JsonMaker) {
        let mut jo = self.to_json();
        if jo.medium.ohlc.len() == 0 {
            return;
        }
        json_maker.set_json_data(&mut jo);

        let pair = &self.cfg.pair;
        let week_id = self.week_id;
        let day_num = self.day_num;

        let title = if day_num == 0 {
            format!("{:?}/{}", &pair, week_id)
        } else {
            format!("{:?}/{}_{}", &pair, week_id, day_num)
        };

        let file_path = if day_num == 0 {
            format!("{}{}/{}.html", OUT_FOLDER, &pair.folder_path(), week_id)
        } else {
            format!(
                "{}{}/{}_{}.html",
                OUT_FOLDER,
                pair.folder_path(),
                week_id,
                day_num
            )
        };

        let json_text = serde_json::to_string_pretty(&jo).unwrap();
        let trades_text = offline::position_html::to_html_table(&self.pos);

        let html_tmpl = std::fs::read_to_string("./src/web/html/ui7.html").unwrap();
        let js_script = std::fs::read_to_string("./src/web/dist/bundle.js").unwrap();
        // JS libs (jQuery and lightweight)
        let jquery = std::fs::read_to_string("./src/web/libs/jquery.min.js").unwrap();
        let lighweight =
            std::fs::read_to_string("./src/web/libs/lightweight-charts.standalone.development.js")
                .unwrap();
        let libs = format!("{} \n {}", jquery, lighweight);

        let html = html_tmpl.replace("{{TITLE}}", &title);
        let html = html.replace("{{JSON_DATA}}", &json_text);
        let html = html.replace("{{TRADE_DATA}}", &trades_text);
        let html = html.replace("{{JS_LIBS}}", &libs);
        let html = html.replace("{{JS_SCRIPT}}", &js_script);

        // Write to file
        let dir = format!("{}{}", OUT_FOLDER, pair.folder_path());

        use std::fs;
        fs::create_dir_all(&dir);
        fs::write(&file_path, html);
        println!("{}", &file_path);
    }
}

fn get_postions_range(pos: &Vec<Position>, time_start: i64, time_end: i64) -> Vec<Position> {
    let time_start = time_start as u64 / 1000;
    let time_end = time_end as u64 / 1000;
    let mut out = vec![];
    for p in pos {
        if p.open_time >= time_start && p.close_time <= time_end {
            out.push(p.clone())
        }
    }
    out
}
