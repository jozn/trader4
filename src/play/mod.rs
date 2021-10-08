use super::forex::loader_csv;
use super::forex::loader_csv::*;
use super::offline;

pub fn play5() {
    use super::candle::*;
    let arr = loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    let mut arr_out = vec![];
    for b in ts.ticking.iter() {
        let cs = offline::tok_kline_to_csv_out(b);
        arr_out.push(cs);
    }

    // let o = serde_json::to_string_pretty(&arr_out).unwrap();
    let o = offline::tok_to_csv_out(&arr_out);

    println!("{:}", o);
}

pub fn play4() {
    use super::candle::*;
    let arr = loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    let mut arr_out = vec![];
    for b in ts.medium.klines_ta.iter() {
        let cs = offline::kline_to_csv_out(b);
        arr_out.push(cs);
    }

    // let o = serde_json::to_string_pretty(&arr_out).unwrap();
    let o = offline::to_csv_out_old(&arr_out);

    println!("{:}", o);
}

pub fn play3() {
    use super::candle::*;
    let arr = loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    ts.print_ticking();
}

pub fn play2() {
    use super::candle::*;
    let arr = loader_csv::_load(50_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    let mut arr_out = vec![];
    for b in ts.medium.klines_ta.iter() {
        let cs = offline::kline_to_csv_out(b);
        arr_out.push(cs);
    }

    // let o = serde_json::to_string_pretty(&arr_out).unwrap();
    let o = offline::to_csv_out_old(&arr_out);

    println!("{:}", o);
}

pub fn play1() {
    use super::candle::*;
    let arr = loader_csv::_load(10000_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time_s: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_ticks(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    ts.print_klines();

    // println!("{:#?}", ts);
}
