use crate::World;
use std::marker::PhantomData;

pub type Score = u32;
pub trait ScoringSystem {
    fn score(w: World) -> Score;
}

pub struct Dharma<S: ScoringSystem> {
    _phantom: PhantomData<S>,
}

impl<S: ScoringSystem> Dharma<S> {}
