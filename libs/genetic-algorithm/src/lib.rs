pub use crate::{selection_method::*, crossover_method::*, mutation_method::*, statistics::*};

pub mod selection_method;
pub mod crossover_method;
pub mod mutation_method;
pub mod statistics;

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

    /// Given a population of individuals, selects, reproduces, and mutates the population.
    pub fn evolve<I>(&self, population: &[I]) -> (Vec<I>, Statistics)
        where I: Individual
    {
        assert!(!population.is_empty());

        let new_population = population
            .iter()
            .map(|_| {
                // Selects two parents and extracts their genome
                let parent_a = self.selection_method.select(population).genome();
                let parent_b = self.selection_method.select(population).genome();

                // Apply crossover method to parents to create the genome of a child
                let mut child = self.crossover_method.crossover(&parent_a, &parent_b);

                // Mutates the child's genome
                self.mutation_method.mutate(&mut child);

                // Convert the genome back to an individual
                I::create(child)
            })
            .collect();

        (new_population, Statistics::new(population))
    }
}

pub trait Individual {
    /// The actual score to the game, similar but not equal to fitness.
    /// For example, the number of apples eaten.
    fn score(&self) -> u32;
    /// The fitness function, two rank the effectivness of an individual's brain.
    /// For example, an expression that combines score and lifetime.
    fn fitness(&self) -> f32;
    /// Convert an individual to its genome, an array that contains weights and biases of the brain.
    fn genome(&self) -> &Vec<f32>;
    /// Convert a genome back to an individual.
    fn create(genom: Vec<f32>) -> Self;
}