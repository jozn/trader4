use csv::{Error, StringRecord};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};

use super::*;
use crate::candle::Tick;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CSVForexRecord {
    pub time: u64,
    pub time_str: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_vol: f64,
    pub ask_vol: f64,
}

impl CSVForexRecord {
    pub fn get_price(&self) -> f64 {
        self.bid_price
    }

    pub fn to_tick(&self) -> Tick {
        let multi = 100_000.;
        Tick {
            time_s: self.time,
            price: self.bid_price * multi,
            price_raw: self.bid_price,
            price_multi: multi,
            qty: 0.0,
            ..Default::default()
        }
    }

    fn from_csv(csv_row: csv::StringRecord) -> Self {
        use std::convert::*;
        let mut i = csv_row.iter();

        let time_str = i.next().unwrap().to_string();

        // let open_time = NaiveDateTime::parse_from_str(&time_str, "yyyymmdd hh:mm:ss:fff").unwrap();
        let open_time = NaiveDateTime::parse_from_str(&time_str, "%Y%m%d %H:%M:%S:%f").unwrap();
        let ots = open_time.format("%Y-%m-%d %H:%M:%S").to_string();

        CSVForexRecord {
            // time: i.next().unwrap().parse().unwrap(),
            // time: open_time.t,
            time: open_time.timestamp() as u64,
            time_str: ots,
            bid_price: i.next().unwrap().parse().unwrap(),
            ask_price: i.next().unwrap().parse().unwrap(),
            bid_vol: i.next().unwrap().parse().unwrap(),
            ask_vol: i.next().unwrap().parse().unwrap(),
        }
    }
}

pub(crate) fn _load(num: u32, file_path: &str) -> Vec<CSVForexRecord> {
    let mut arr = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1000, file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    // let mut rdr = csv::Reader::from_reader(reader);
    let mut i = 0;
    for result in rdr.records().take(num as usize) {
        match result {
            Ok(csv_row) => {
                // println!("csv: {:?}", &csv_row );

                let row = CSVForexRecord::from_csv(csv_row);
                let r = arr.push(row);
                // println!("err :{:?}", r );

                i += 1;
            }
            Err(_) => {}
        }
    }
    // println!("num :{}", i);

    arr
}
