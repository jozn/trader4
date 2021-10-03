use super::*;

pub fn play1() {
    let mut n = crate::sim::Runner::new();
    n.run();
}

pub fn play2() {
    let arr = _load(10_000, "/media/hamid/K/forex1/EURUSD_tab3.csv");

    println!("arr {:#?}", arr);
}
