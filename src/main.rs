extern crate openbrahma;

use openbrahma::geometry::{Circle, Color, Transform, Vector};
use openbrahma::{Brain, Creature, Object, World};

fn run<F>(mut f: F)
where
    F: FnMut() -> (),
{
    use std::io::{self, Write};
    use std::time::Instant;
    let mut iter = 0;
    let mut before = Instant::now();
    loop {
        f();
        iter += 1;
        let now = Instant::now();
        if (now - before).as_secs() >= 1 {
            print!("\r{} updates per second...", iter);
            io::stdout().flush().unwrap();
            before = now;
            iter = 0;
        }
    }
}

fn main() {
    println!("Hello, Dharma!");

    let mut w = World::new();
    w.add_object(
        Box::new(Creature::new(
            100,
            Brain::new(10000, 100),
            Circle {
                r: 20.0,
                col: Color(0.5),
            },
        )),
        Transform {
            pos: Vector(0.0, 0.0),
            rot: 0.0,
        },
    );

    run(|| w.update());
}
