use trader3;
use trader3::run::WorldRunner;

fn main() {
    let mut w = trader3::world::HullWorld::new_runner();
    w.run();
}

fn m1() {
    let w = trader3::world::HullWorld::new();
    let mut wr = WorldRunner::new(w);

    wr.run();
}
