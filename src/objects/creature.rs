use crate::geometry::{Ray, Shape, Transform, Vector};
use crate::io::{Actuator, Collide, Die, Eat, End, Eye, Move, Sense, Sensor};
use crate::Genetic;
use crate::{Axon, Brain, Corpus, NeuronId};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Clone, Serialize, Deserialize)]
pub struct Creature {
    pub health: u32,
    brain: Brain,
    body: Shape,
    eye: Vec<Axon>,
    motors: Vec<NeuronId>,
    danger: Vec<Axon>,
}

impl Creature {
    pub fn new(health: u32, brain: Brain, body: Shape) -> Creature {
        let eye = brain.random_axons(10);
        let motors = brain.random_neurons(4); // Forward, Backward, Rotate Left, Rotate Right
        let danger = brain.random_axons(5);
        Creature {
            health,
            brain,
            body,
            eye,
            motors,
            danger,
        }
    }
}

impl Genetic for Creature {
    fn crossover(&mut self, other: &Creature) {
        self.brain.crossover(&other.brain);
    }
    fn mutate(&mut self, rate: f32) {
        self.brain.mutate(rate);
    }
}

impl Corpus for Creature {
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>> {
        let mut actuators = Vec::<Box<dyn Actuator>>::new();

        self.health -= 1;
        if self.health == 0 {
            actuators.push(Box::new(Die));
            actuators.push(Box::new(End));
            return actuators;
        }

        self.brain.update();

        if self.health < 1000 {
            for (pow, id) in self.danger.iter() {
                self.brain.stimulate(*id, *pow);
            }
        }

        for sense in senses {
            match sense {
                Sense::Vision(pixels) => {
                    for ((pow, id), pixel) in self.eye.iter().zip(pixels.iter()) {
                        if pixel.is_some() {
                            self.brain.stimulate(*id, *pow);
                        }
                    }
                }
                Sense::Collision(ids) => {
                    ids.iter().for_each(|&id| {
                        actuators.push(Box::new(Eat { id }));
                    });
                }
            }
        }
        let motor_deltas = self.brain.get_deltas(&self.motors);
        let forward = Vector::i() * ((motor_deltas[1] - motor_deltas[0]) as f64);
        let rot = (motor_deltas[2] - motor_deltas[3]) as f64;
        actuators.push(Box::new(Move {
            trans: Transform {
                trans: forward,
                rot: rot,
            },
        }));
        actuators
    }
    fn shape(&self) -> &Shape {
        &self.body
    }
    fn sensors(&self) -> Vec<Box<dyn Sensor>> {
        vec![
            Box::new(Eye {
                ray: Ray {
                    pos: Vector::zero(),
                    ang: 0.0,
                },
                fov: 0.5,
                res: 10,
            }),
            Box::new(Collide),
        ]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
