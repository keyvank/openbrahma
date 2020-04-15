use crate::geometry::{Color, Shape, Transform, Vector};
use crate::objects::{Creature, Food};
use crate::{Genetic, World};
use rayon::prelude::*;
use std::marker::PhantomData;

pub type Score = u32;
pub trait ScoringSystem {
    fn score(c: Box<Creature>) -> Score;
}
pub struct LifespanScoring;
impl ScoringSystem for LifespanScoring {
    fn score(c: Box<Creature>) -> Score {
        let mut lifespan = 0;
        for _ in 0..10 {
            let mut w = World::new();
            w.add_object(
                c.clone(),
                Transform {
                    trans: Vector::zero(),
                    rot: 0.0,
                },
            );
            for _ in 0..1 {
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
            while w.update() {
                lifespan += 1;
            }
        }

        lifespan / 10
    }
}

pub struct Dharma<S: ScoringSystem> {
    creatures: Vec<(Creature, u32)>,
    _phantom: PhantomData<S>,
}

impl<S: ScoringSystem> Dharma<S> {
    pub fn new(population: usize, starter: Creature) -> Dharma<S> {
        Dharma {
            creatures: vec![(starter, 0); population],
            _phantom: PhantomData::<S>,
        }
    }
    pub fn cycle(&mut self) -> Creature {
        self.creatures.par_iter_mut().for_each(|(c, score)| {
            *score = S::score(Box::new(c.clone()));
        });
        self.creatures.sort_by_key(|(_, score)| -(*score as i32));
        self.creatures.drain(6..);
        for i in 0..6 {
            for j in i + 1..6 {
                let mut c = self.creatures[i].0.clone();
                c.crossover(&self.creatures[j].0);
                let mut c2 = self.creatures[j].0.clone();
                c2.crossover(&self.creatures[i].0);
                self.creatures.push((c, 0));
                self.creatures.push((c2, 0));
            }
            let mut c = self.creatures[i].0.clone();
            c.mutate(0.1);
            self.creatures.push((c, 0));
        }
        println!(
            "Best score: {}",
            S::score(Box::new(self.creatures[0].0.clone()))
        );

        self.creatures[0].0.clone()
    }
}
