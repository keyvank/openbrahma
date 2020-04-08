use crate::geometry::{Ray, Shape, Transform, Vector};
use crate::io::{Actuator, Collide, Eat, Eye, Move, Sense, Sensor};
use crate::{Brain, Corpus, NeuronId};

pub struct Creature<S: Shape> {
    health: u32,
    brain: Brain,
    body: S,
    eye: Vec<NeuronId>,
    motors: Vec<NeuronId>,
}

impl<S: Shape> Creature<S> {
    pub fn new(health: u32, brain: Brain, body: S) -> Creature<S> {
        let eye = brain.random_neurons(10);
        let motors = brain.random_neurons(4); // Forward, Backward, Rotate Left, Rotate Right
        Creature {
            health,
            brain,
            body,
            eye,
            motors,
        }
    }
}

impl<S: Shape> Corpus for Creature<S> {
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Box<dyn Actuator>> {
        let mut actuators = Vec::<Box<dyn Actuator>>::new();
        self.brain.update();
        for sense in senses {
            match sense {
                Sense::Vision(pixels) => {
                    for (&neuron, pixel) in self.eye.iter().zip(pixels.iter()) {
                        if pixel.is_some() {
                            self.brain.stimulate(neuron, 3i32);
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
    fn shape(&self) -> &dyn Shape {
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
}
