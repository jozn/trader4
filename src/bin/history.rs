use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use trader3;
use trader3::collector::downloader::Downloader;
use trader3::configs::assets;
use trader3::configs::assets::Pair;
use trader3::core::helper;
use trader3::online;
use trader3::online::ctrader::{CTrader, Config};

fn main() {
    run();
}

const YEAR_ZERO_WEEK: i64 = 1609632000_000; // Sunday, 3 January 2021 00:00:00
const START_WEEK: i64 = YEAR_ZERO_WEEK + 24 * MS_IN_WEEK; // 3 Jan 2021
const MS_IN_WEEK: i64 = 7 * 86400_000;

fn run() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "lcd2-Q0fEVUMMILzkyjwcG1YKmj1vsI1HFRZJx5ETCw".to_string(),
        ctid: 23382349, // pepperstone
                        // ctid: 22851452, // ctrader demo account data
    };

    // 5 is the limit of cTrader concurrent histroical data call.
    // https://spotware.github.io/open-api-docs/protocol-buffers/#limitations
    let pool = ThreadPool::new(5);

    let mut pair_ids = assets::get_all_symbols_ids();
    pair_ids.reverse(); // TEMP: first stocks
                        // let pair_ids = vec![43];// todo: remove
                        // let pair_ids = vec![10099,10096,10011,41];// gold
    for pair_id in pair_ids {
        let pair = assets::Pair::id_to_symbol(pair_id);
        let cfg = cfg.clone();
        // thread::spawn(move || {
        pool.execute(move || {
            let week_ids = get_weeks(&pair);
            if week_ids.len() >= 2 {
                let mut downloader = Downloader::new(&pair, &cfg);
                for week_id in week_ids {
                    if downloader.disconnected {
                        downloader = Downloader::new(&pair, &cfg);
                    }
                    let folder = get_folder_path(&pair);
                    let file_path = get_file_path(&pair, week_id);

                    println!(
                        "{:?} - {} - Week {} starting",
                        pair,
                        helper::time_tag_string(),
                        week_id
                    );
                    let (week_time_start, week_time_end) = get_weeks_times(week_id);
                    let (csv_res, is_err) = downloader.get_data(week_time_start, week_time_end);
                    if is_err {
                        println!("{:?} > Pre-mature end retrying.", pair);
                        std::thread::sleep(Duration::from_millis(10000));
                        continue;
                    }

                    if csv_res.len() > 0 {
                        std::fs::create_dir_all(&folder).unwrap();
                        std::fs::write(&file_path, &csv_res);
                    }

                    println!(
                        "{:?} > Completed Week {} - Size: {} -  Path: {} ",
                        pair,
                        week_id,
                        csv_res.len(),
                        file_path
                    );

                    std::thread::sleep(Duration::from_millis(4000));
                }
            }
        });
    }

    std::thread::sleep(std::time::Duration::new(20_000_000, 0));
}

fn get_weeks_num(seconds: i64) -> i64 {
    (seconds - YEAR_ZERO_WEEK) / (86400_000 * 7) + 1
}

fn get_weeks_times(weed_id: i64) -> (i64, i64) {
    let start = YEAR_ZERO_WEEK + (weed_id - 1) * MS_IN_WEEK;
    (start, start + MS_IN_WEEK)
}

fn get_file_path(pair: &Pair, weeks_num: i64) -> String {
    let cat = pair.to_category();
    let s = format!("./data/{}/{:?}/{}.tsv", cat, pair, weeks_num);
    s
}

fn get_folder_path(pair: &Pair) -> String {
    let cat = pair.to_category();
    let s = format!("./data/{}/{:?}", cat, pair);
    s
}

// todo simplify it's logic
fn get_weeks(pair: &Pair) -> Vec<i64> {
    let mut week_ids = vec![];
    let mut current_week_start_ms = START_WEEK;
    loop {
        let week_id = get_weeks_num(current_week_start_ms);

        let folder = get_folder_path(&pair);
        let file_path = get_file_path(&pair, week_id);

        // Old legacy .csv to .tsv file rename
        let old_csv_file = file_path.replace(".tsv", ".csv");
        if std::path::Path::new(&old_csv_file).exists() {
            std::fs::rename(&old_csv_file, &file_path);
        }
        // If we already downloaded the week pair data?
        if std::path::Path::new(&file_path).exists() {
            println!(
                "{:?} - {} - Week {} Ignored (downloaded already)",
                pair,
                helper::time_tag_string(),
                week_id
            );
            current_week_start_ms += MS_IN_WEEK;
            if current_week_start_ms >= helper::get_time_ms() as i64 {
                println!("{:?} > Finished! #1", pair);
                break; // End the pair dls
            } else {
                continue; // Go next week
            }
        }

        // A work around, pepperstone does not have stocks below 42
        if pair.is_us_forex() {
            if week_id >= 42 {
                week_ids.push(week_id)
            }
        } else {
            week_ids.push(week_id);
        }
        current_week_start_ms += MS_IN_WEEK;
        if current_week_start_ms >= helper::get_time_ms() as i64 {
            // println!("{:?} > Finished! #2", pair);
            break;
        }
    }

    week_ids
}
