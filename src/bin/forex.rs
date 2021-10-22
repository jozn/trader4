use trader2;

fn main() {
    // trader2::offline.play::play1();
    // trader2::offline.play::play2();
    // trader2::offline.play::play3();
    // trader2::offline.play::play5();

    run_sim();
}

pub fn run_sim() {
    let mut n = trader2::sim::Runner::new();
    n.run();
}

fn main_2() {
    trader2::forex::play::play1();
}
