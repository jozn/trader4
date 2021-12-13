use trader3;

fn main() {
    trader3::optimizer::run1();

    let pcfg = trader3::optimizer::get_all_candle_cfgs();
    // println!("{:#?}", &pcfg);
    println!("{:#?}", pcfg.len());

    // trader3::offline::run::run2();
}
