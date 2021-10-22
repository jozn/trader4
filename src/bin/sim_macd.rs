use trader2;

fn main() {
    run_sim();
}

pub fn run_sim() {
    let mut n = trader2::offline::world::sim_macd::SimMacdWorld::new_runner();
    n.run();
}
