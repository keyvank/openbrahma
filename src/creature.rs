use super::brain::{Brain, NeuronId};
use super::world::Object;

pub struct Creature<O: Object> {
    health: u32,
    brain: Brain,
    body: O,
    eye: Vec<NeuronId>,
    motors: Vec<NeuronId>,
}

impl<O: Object> Creature<O> {
    pub fn new(health: u32, brain: Brain, body: O) -> Creature<O> {
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

    pub fn tick(&mut self) {
        self.brain.tick();
        for &e in self.eye.iter() {
            self.brain.stimulate(e, 3i32);
        }
        let motor_deltas = self.brain.get_deltas(&self.motors);
    }
}
