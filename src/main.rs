extern crate glutin_window;
extern crate graphics;
extern crate openbrahma;
extern crate opengl_graphics;
extern crate piston;

use openbrahma::dharma::{Dharma, LifespanScoring};
use openbrahma::geometry::{Circle, Color, Transform, Vector};
use openbrahma::objects::{Creature, Food};
use openbrahma::{Brain, World};

mod graphical;

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

    let mut d = Dharma::<LifespanScoring>::new();
    let creature = d.best_creature();

    //graphical::simulate(&mut w);
}
