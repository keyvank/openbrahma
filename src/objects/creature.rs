use crate::geometry::{Ray, Shape, Transform, Vector};
use crate::io::{Actuator, Collide, Die, Eat, End, Eye, Move, Sense, Sensor};
use crate::Genetic;
use crate::{Axon, Brain, Corpus, NeuronId, THRESHOLD};
use rand::{thread_rng, Rng};
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
    pub fn save(&self, path: &str) {
        use std::fs;
        fs::write(path, serde_json::to_string(&self).unwrap()).unwrap();
    }
    pub fn load(path: &str) -> Self {
        use std::fs;
        let s = fs::read_to_string(path).unwrap();
        serde_json::from_str(&s).unwrap()
    }
}

impl Genetic for Creature {
    fn crossover(&mut self, other: &Creature) {
        self.brain.crossover(&other.brain);
        self.brain.crossover_axons(&mut self.eye, &other.eye);
        self.brain.crossover_axons(&mut self.danger, &other.danger);
        self.brain
            .crossover_neurons(&mut self.motors, &other.motors);
    }
    fn mutate(&mut self, rate: f32) {
        self.brain.mutate(rate);
        self.brain.mutate_axons(&mut self.eye, rate);
        self.brain.mutate_axons(&mut self.danger, rate);
        self.brain.mutate_neurons(&mut self.motors, rate);
    }
}

impl Corpus for Creature {
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>> {
        let mut actuators = Vec::<Box<dyn Actuator>>::new();

        self.health -= self.health.min(1);
        if self.health == 0 {
            actuators.push(Box::new(Die));
            actuators.push(Box::new(End));
            return actuators;
        }

        self.brain.update();

        if self.health < 2000 {
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
        let change: u32 = motor_deltas.iter().map(|n| n.abs() as u32).sum::<u32>() / 10;
        self.health -= self.health.min(change);
        let forward = Vector::i() * ((motor_deltas[1] - motor_deltas[0]) as f64);
        let rot = (motor_deltas[2] - motor_deltas[3]) as f64;
        actuators.push(Box::new(Move {
            trans: Transform {
                trans: forward,
                rot: rot / 10.0,
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
