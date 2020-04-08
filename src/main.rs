extern crate glutin_window;
extern crate graphics;
extern crate openbrahma;
extern crate opengl_graphics;
extern crate piston;

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

    let mut w = World::new();
    w.add_object(
        Box::new(Creature::new(
            10000,
            Brain::new(1000, 100),
            Box::new(Circle {
                r: 20.0,
                col: Color::white(),
            }),
        )),
        Transform {
            trans: Vector::zero(),
            rot: 0.0,
        },
    );
    w.add_object(
        Box::new(Food::new(
            5000,
            Box::new(Circle {
                r: 10.0,
                col: Color::blue(),
            }),
        )),
        Transform {
            trans: Vector(70.0, 40.0),
            rot: 0.0,
        },
    );

    graphical::simulate(&mut w);
}
