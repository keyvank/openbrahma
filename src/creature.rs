use super::brain::{Brain, NeuronId};
use super::world::Object;

pub struct Creature<O: Object> {
    health: u32,
    brain: Brain,
    body: O,
    eye: Vec<NeuronId>,
}

impl<O: Object> Creature<O> {
    pub fn new(health: u32, brain: Brain, body: O) -> Creature<O> {
        let eye = brain.random_nodes(10);
        Creature {
            health,
            brain,
            body,
            eye,
        }
    }

    pub fn tick(&mut self) {
        self.brain.tick();
        for &e in self.eye.iter() {
            self.brain.stimulate(e, 0i32);
        }
    }
}
