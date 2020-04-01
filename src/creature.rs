use super::brain::{Brain, NeuronId};
use super::sense::{Eye, Sense, Sensor};
use super::shape::{Shape, Transform};
use super::vector::Vector;
use super::world::{Action, Updatable};

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

impl<S: Shape> Updatable for Creature<S> {
    fn update(&mut self, senses: &Vec<Sense>) -> Vec<Action> {
        self.brain.update();
        for sense in senses {
            match sense {
                Sense::Vision(pixels) => {
                    for (&neuron, pixel) in self.eye.iter().zip(pixels.iter()) {
                        if pixel.is_none() {
                            self.brain.stimulate(neuron, 3i32);
                        }
                    }
                }
            }
        }
        let motor_deltas = self.brain.get_deltas(&self.motors);
        let forward = Vector(1.0, 0.0) * ((motor_deltas[1] - motor_deltas[0]) as f64);
        let rot = (motor_deltas[2] - motor_deltas[3]) as f64;
        vec![Action::Move(Transform {
            pos: forward,
            rot: rot,
        })]
    }
    fn shape(&self) -> &dyn Shape {
        &self.body
    }
    fn sensors(&self) -> Vec<Box<dyn Sensor>> {
        vec![Box::new(Eye { fov: 0.5, res: 10 })]
    }
}
