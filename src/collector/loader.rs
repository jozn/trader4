use crate::collector::row_data::{BTickData, TickBinFast};
use crate::configs::assets::Pair;
use csv::{Error, StringRecord};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Range;

impl BTickData {
    // Note: probelby manulay of this is unneccessory but as we we alerady implemented this
    //  for other structs it's good. Serde will works prefectely for this type.
    fn from_csv(csv_row: csv::StringRecord, pair: &Pair) -> Self {
        use std::convert::*;
        let mut i = csv_row.iter();

        Self {
            pair: pair.clone(),
            date_str: i.next().unwrap().to_string(),
            timestamp_sec: i.next().unwrap().parse().unwrap(),
            timestamp: i.next().unwrap().parse().unwrap(),
            bid_price: i.next().unwrap().parse().unwrap(),
            ask_price: i.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn load_tsv_rows(file_path: &str, pair: &Pair) -> Vec<BTickData> {
    let mut arr = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1000, file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    let mut i = 0;
    for result in rdr.records() {
        match result {
            Ok(csv_row) => {
                let row = BTickData::from_csv(csv_row, pair);
                let r = arr.push(row);

                i += 1;
            }
            Err(_) => {}
        }
    }
    // println!("num :{}", i);
    arr
}

pub fn load_week(pair: &Pair, week_id: u16) -> Vec<BTickData> {
    load_weeks(pair, week_id..(week_id + 1))
}

pub fn load_weeks(pair: &Pair, rng: Range<u16>) -> Vec<BTickData> {
    let mut arr = vec![];
    for i in rng {
        let path = format!("/mnt/t/trader/data_fast/{}/{}.bin", pair.folder_path(), i);
        if std::path::Path::new(&path).exists() {
            let ticks = load_rows_fast(&path, pair);
            for t in ticks {
                arr.push(t);
            }
        }
    }
    arr
}

// Fast fns
pub fn load_rows_fast(file_path: &str, pair: &Pair) -> Vec<BTickData> {
    let file = std::fs::read(file_path).unwrap();
    let arr_bins: Vec<TickBinFast> = bincode::deserialize(&file).unwrap();

    let mut i = 0;
    let mut out_arr = Vec::with_capacity(arr_bins.len());
    for b in arr_bins {
        out_arr.push(b.to_tick(pair));
    }
    // println!("num :{}", i);
    out_arr
}
