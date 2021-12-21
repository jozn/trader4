use crate::candle::Tick;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use csv::{Error, StringRecord};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Range;

impl BTickData {
    pub fn to_tick(&self) -> Tick {
        let multi = 100_000.;
        Tick {
            time_s: self.timestamp_sec as u64,
            // price_raw: self.bid_price * multi,
            price_raw: self.bid_price,
            multi: 1.,
            qty: 0.0,
            timestamp: self.timestamp,
            bid_price: self.bid_price,
            ask_price: self.ask_price,
        }
    }

    // Note: probelby manulay of this is unneccessory but as we we alerady implemented this
    //  for other structs it's good. Serde will works prefectely for this type.
    fn from_csv(csv_row: csv::StringRecord) -> Self {
        use std::convert::*;
        let mut i = csv_row.iter();

        Self {
            date_str: i.next().unwrap().to_string(),
            timestamp_sec: i.next().unwrap().parse().unwrap(),
            timestamp: i.next().unwrap().parse().unwrap(),
            bid_price: i.next().unwrap().parse().unwrap(),
            ask_price: i.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn load_rows(file_path: &str) -> Vec<BTickData> {
    let mut arr = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1000, file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    // let mut rdr = csv::Reader::from_reader(reader);
    let mut i = 0;
    for result in rdr.records() {
        match result {
            Ok(csv_row) => {
                // println!("csv: {:?}", &csv_row );

                let row = BTickData::from_csv(csv_row);
                // let row =
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

pub fn load_ticks(file_path: &str) -> Vec<Tick> {
    let arr = load_rows(file_path);
    let res = arr.iter().map(|v| v.to_tick()).collect();
    res
}

pub fn load_week(pair: &Pair, week_id: u16) -> Vec<BTickData> {
    load_all_pair(pair, week_id..(week_id + 1))
}

pub fn load_all_pair(pair: &Pair, rng: Range<u16>) -> Vec<BTickData> {
    let mut arr = vec![];
    for i in rng {
        let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, i);
        if std::path::Path::new(&path).exists() {
            let ticks = load_rows(&path);
            for t in ticks {
                arr.push(t);
            }
        }
    }
    arr
}
