mod brain;
pub mod dharma;
pub mod geometry;
pub mod io;
pub mod objects;
mod world;

pub use brain::*;
pub use world::*;

pub trait Genetic {
    fn crossover(&mut self, other: &Self);
    fn mutate(&mut self, rate: f32);
}
