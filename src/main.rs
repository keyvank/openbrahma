extern crate glutin_window;
extern crate graphics;
extern crate openbrahma;
extern crate opengl_graphics;
extern crate piston;

use openbrahma::dharma::{Dharma, LifespanScoring};
use openbrahma::geometry::{Color, Shape, Transform, Vector};
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

    let mut d = Dharma::<LifespanScoring>::new(100, Creature::load("creatures/eater.json"));

    let mut i = 0;
    for _ in 0..100 {
        d.cycle();
        println!("{}", i);
        i += 1;
    }
    let creature = d.cycle();
    creature.save("creature.json");

    let mut w = World::new();
    w.add_object(
        Box::new(creature.clone()),
        Transform {
            trans: Vector::zero(),
            rot: 0.0,
        },
    );
    for _ in 0..20 {
        w.add_object(
            Box::new(Food::new(
                500,
                Shape::Circle {
                    r: 10.0,
                    col: Color::blue(),
                },
            )),
            Transform {
                trans: Vector(
                    (rand::random::<f64>() - 0.5) * 500.0,
                    (rand::random::<f64>() - 0.5) * 500.0,
                ),
                rot: 0.0,
            },
        );
    }

    graphical::simulate(&mut w);
}
