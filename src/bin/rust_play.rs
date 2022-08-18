use std::borrow::BorrowMut;
use std::cell::Cell;
// use trader4;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    rc();
    arc();

    let c = Cell;
}

struct Abc {
    num: i64,
}

fn rc() {
    println!("start of rc");
    let mut rc = Rc::new(Abc { num: 1 });
    for i in 0..100_000_000 {
        // let r = rc.borrow_mut();
        let r = Rc::get_mut(&mut rc).unwrap();
        // let r = rc.get_mut();
        r.num += 1;
        if i % 5_000_000 == 0 {
            println!("rc {}", i);
        }
        // rc.num +=1;
    }
    println!("end of rc");
}

fn arc() {
    println!("start of arc");
    let mut rc = Arc::new(Abc { num: 1 });
    for i in 0..100_000_000 {
        let r = Arc::get_mut(&mut rc).unwrap();
        r.num += 1;
        if i % 5_000_000 == 0 {
            println!("arc {}", i);
        }
    }
    println!("end of arc");
}
