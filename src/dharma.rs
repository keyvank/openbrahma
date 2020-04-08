use crate::geometry::{Circle, Color, Transform, Vector};
use crate::objects::{Creature, Food};
use crate::World;
use std::marker::PhantomData;

pub type Score = u32;
pub trait ScoringSystem {
    fn score(c: Box<Creature>) -> Score;
}
pub struct LifespanScoring;
impl ScoringSystem for LifespanScoring {
    fn score(c: Box<Creature>) -> Score {
        let mut w = World::new();
        w.add_object(
            c,
            Transform {
                trans: Vector::zero(),
                rot: 0.0,
            },
        );
        w.add_object(
            Box::new(Food::new(
                50,
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

        0
    }
}

pub struct Dharma<S: ScoringSystem> {
    _phantom: PhantomData<S>,
}

impl<S: ScoringSystem> Dharma<S> {}
