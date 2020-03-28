use super::brain::Brain;
use super::world::Object;

pub struct Creature<O: Object> {
    brain: Brain,
    body: O,
}
