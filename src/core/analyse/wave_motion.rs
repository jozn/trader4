use crate::ta::WaveRes;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

// This module analyses 3 price waves

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionGen {
    pub major: Vec<WaveRes>,
    pub medium: Vec<WaveRes>,
    pub small: Vec<WaveRes>,
}

impl MotionGen {
    pub fn new(
        waves_major: &Vec<WaveRes>,
        waves_medium: &Vec<WaveRes>,
        waves_small: &Vec<WaveRes>,
    ) -> Self {
        MotionGen {
            major: waves_major.clone(),
            medium: waves_medium.clone(),
            small: waves_small.clone(),
        }
    }

    pub fn run(&self) -> Vec<Motion> {
        let mut mots = gen_motion(&self.major);
        for mut mot in &mut mots {
            // For medium waves
            mot.sub_motions = gen_motion(&get_range_wave_res(
                &self.medium,
                mot.start_time,
                mot.end_time,
            ));
            mot.sub_cnt = mot.sub_motions.len() as i32;

            // For small waves
            for mut mot_med in &mut mot.sub_motions {
                mot_med.sub_motions = gen_motion(&get_range_wave_res(
                    &self.small,
                    mot_med.start_time,
                    mot_med.end_time,
                ));
                mot_med.sub_cnt = mot_med.sub_motions.len() as i32;
            }
        }
        mots
    }
}

fn get_range_wave_res(waves: &Vec<WaveRes>, start: i64, end: i64) -> Vec<WaveRes> {
    let mut arr = vec![];
    for w in waves {
        if w.time >= start && w.time <= end {
            arr.push(w.clone())
        }
    }
    arr
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub mid: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub dur_sec: i64,
    pub dir: Dir,
    pub start_price: f64,
    pub end_price: f64,
    pub price_diff: f64,
    pub price_diff_per: f64,
    pub sub_cnt: i32,
    pub sub_motions: Vec<Motion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dir {
    Up,
    Down,
}

pub fn gen_motion(waves: &Vec<WaveRes>) -> Vec<Motion> {
    let mut arr = vec![];
    let mut win = waves.windows(2);
    let mut mid = 0;
    for wn in win {
        let w0 = wn.get(0).unwrap();
        let w1 = wn.get(1).unwrap();

        let dir = if w0.price > w1.price {
            Dir::Down
        } else {
            Dir::Up
        };

        let pdif = (w1.price - w0.price);
        let motion = Motion {
            mid: mid,
            start_time: w0.time, //1000,
            end_time: w1.time,   //1000,
            dur_sec: (w1.time - w0.time) / 1000,
            dir,
            start_price: w0.price,
            end_price: w1.price,
            price_diff: pdif,
            price_diff_per: 100. * pdif / w0.price,
            sub_cnt: 0,
            sub_motions: vec![],
        };
        mid += 1;
        arr.push(motion)
    }

    arr
}
