use super::*;
use crate::bar::*;
use crate::offline::Position;
use crate::ta::zigzag::{ZigZag, ZigZagRes};
use crate::ta::Wave;
use crate::{analyse, offline};
use std::os::unix::raw::off_t;

use crate::core::json_output::*;

// todo: extract this to general funs with taking major,medium and small
pub fn sky_eng_to_json(sky: &SkyEng, start: i64, end: i64, pos: &Vec<Position>) -> SkyJsonOut {
    let s = sky;
    let mut out = SkyJsonOut::default();
    out.major = bars_to_json_old(s.major_bars.get_bars_ph(start, end));
    out.medium = bars_to_json_old(s.medium_bars.get_bars_ph(start, end));
    out.small = bars_to_json_old(s.small_bars.get_bars_ph(start, end));

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

    //////////// Motion Analyse
    use crate::core::analyse::wave_motion;
    // let mots = analyse::gen_motion(&wave3.wave_ress);
    let mo_gen = wave_motion::MotionGen::new(&wave3.wave_ress, &wave2.wave_ress, &wave1.wave_ress);
    // let mo_gen = analyse::MotionGen::new(&wave3.wave_ress,&wave1.wave_ress,&vec![]);
    let mots = mo_gen.run();
    // println!("mots: {:#?}", mots);
    ///////////

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

pub fn sky_eng_to_trend_analyse(
    sky: &SkyEng,
    start: i64,
    end: i64,
    pos: &Vec<Position>,
) -> TrendAnalyseOut {
    println!("========================");
    let mut tao = TrendAnalyseOut::default();
    let s = sky;
    for b in &s.medium_bars.bars_primary {
        // println!("====> {:}", b.primary.ta.vel.end_vel_pip);
        tao.tt.push(b.big.ta.vel.end_vel_pip);
    }
    tao
}
