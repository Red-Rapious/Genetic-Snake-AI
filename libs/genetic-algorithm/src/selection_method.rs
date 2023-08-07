use crate::*;
use rand::seq::SliceRandom;

pub trait SelectionMethod {
    /// Retrieves one individual from population, while taking into account the fitnesses of individuals.
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
    /// The probability of an individual to be selected is proportionnal to their fitness.
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I 
    where I: Individual
    {
        population
            .choose_weighted(&mut rand::thread_rng(), |individual| individual.fitness())
            .expect("[ERROR] Empty population, or every fitness is zero.")
    }
}