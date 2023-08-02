use crate::{selection_method::*, crossover_method::*, mutation_method::*};

pub mod selection_method;
pub mod crossover_method;
pub mod mutation_method;

pub struct GeneticAlgorithm<S> 
    where S: SelectionMethod
{
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>
}

pub trait Individual {
    fn genome(&self) -> Vec<f32>;
    fn fitness(&self) -> f32;
    fn create(chromosome: Vec<f32>) -> Self;
}