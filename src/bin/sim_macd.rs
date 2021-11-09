use trader2;

fn main() {
    // run_sim();
    run_all();
}

pub fn run_all() {
    let mut n = trader2::offline::world::sim_macd::SimMacdWorld::run_all();
}

pub fn run_sim() {
    let mut n = trader2::offline::world::sim_macd::SimMacdWorld::new_runner();
    n.run();
}
