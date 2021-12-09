use trader3;

fn main() {
    // trader3::offline_old.play::play1();
    // trader3::offline_old.play::play2();
    // trader3::offline_old.play::play3();
    // trader3::offline_old.play::play5();

    run_sim();
}

pub fn run_sim() {
    let mut n = trader3::sim::Runner::new();
    n.run();
}

fn main_2() {
    trader3::loader::play::play1();
}
