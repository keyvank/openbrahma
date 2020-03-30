use super::brain::{Brain, NeuronId};
use super::shape::Shape;
use super::world::{Action, Updatable, World};

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
    fn update(&mut self) -> Vec<Action> {
        self.brain.update();
        for &e in self.eye.iter() {
            self.brain.stimulate(e, 3i32);
        }
        let motor_deltas = self.brain.get_deltas(&self.motors);
        Vec::new()
    }
}
