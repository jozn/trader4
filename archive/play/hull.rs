use trader2;
use trader2::run::WorldRunner;

fn main() {
    let mut w = trader2::world::HullWorld::new_runner();
    w.run();
}

fn m1() {
    let w = trader2::world::HullWorld::new();
    let mut wr = WorldRunner::new(w);

    wr.run();
}
