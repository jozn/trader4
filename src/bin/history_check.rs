mod trans_dc;

use std::path::Path;
use trader3::collector;

pub fn main() {
    let pairs = trader3::configs::assets::get_all_symbols();

    for pair in pairs {
        for i in 1..=53 {
            let path = format!("/mnt/c/me/data/{:?}/{}.tsv", pair, i);
            if std::path::Path::new(&path).exists() {
                let ticks = collector::loader::load_rows(&path);

                let first = ticks.first().unwrap();
                let last = ticks.last().unwrap();

                let valid = (last.timestamp_sec - first.timestamp_sec) > 4 * 86_400;
                if !valid {
                    println!(
                        "{} - {} - Invalid ===============================",
                        i,
                        ticks.len()
                    );
                }

                let mut max_diff = 0;
                let mut pre = first;

                for t in &ticks {
                    let diff = t.timestamp_sec - pre.timestamp_sec;
                    if diff > max_diff {
                        max_diff = diff;
                    }
                    pre = t;
                }

                println!(
                    "{:?} - {}.tsv - len = {} - max ticks diff = {}",
                    pair,
                    i,
                    ticks.len(),
                    max_diff
                );
            }
        }
    }
}
