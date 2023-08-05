use crate::*;
use rand::seq::SliceRandom;

pub trait SelectionMethod {
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I 
    where I: Individual
    {
        population
            .choose_weighted(&mut rand::thread_rng(), |individual| individual.fitness())
            .expect("[ERROR] Empty population, or every fitness is zero.")
    }
}