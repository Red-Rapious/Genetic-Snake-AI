pub use crate::{selection_method::*, crossover_method::*, mutation_method::*};

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

impl<S> GeneticAlgorithm<S> 
    where S: SelectionMethod 
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static
        ) -> Self {
        Self { 
            selection_method, 
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method)
        }
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
        where I: Individual
    {
        assert!(!population.is_empty());

        population
            .iter()
            .map(|_| {
                let parent_a = self.selection_method.select(population).genome();
                let parent_b = self.selection_method.select(population).genome();

                let mut child = self.crossover_method.crossover(parent_a, parent_b);

                self.mutation_method.mutate(&mut child);

                I::create(child)
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn genome(&self) -> &Vec<f32>;
    fn create(genom: Vec<f32>) -> Self;
}