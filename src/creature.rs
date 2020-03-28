use super::brain::Brain;
use super::world::Object;

pub struct Creature<O: Object> {
    health: u32,
    brain: Brain,
    body: O,
}

impl<O: Object> Creature<O> {
    pub fn new(health: u32, brain: Brain, body: O) -> Creature<O> {
        Creature {
            health,
            brain,
            body,
        }
    }
}
