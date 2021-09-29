use super::forex::loader_csv;
use super::forex::loader_csv::*;

pub fn play1() {
    use super::candle::*;
    let arr = loader_csv::_load(10_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    let mut ts = super::candle::CandleSeriesTA::new();
    let mut ticks_arr = TimeSerVec::new();
    let mut i = 0;
    for v in arr {
        let tt = super::candle::Tick {
            time: v.time,
            price: v.ask_price * 100_000.,
            qty: 0.0,
        };
        ticks_arr.push(tt);
        i += 1;

        if i == 50 {
            ts.add_trades(ticks_arr.clone());
            i = 0;
            ticks_arr.clear();
        }
    }

    ts.print_klines();

    // println!("{:#?}", ts);
}
