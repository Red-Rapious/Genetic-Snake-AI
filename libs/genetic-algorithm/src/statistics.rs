use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    average_fitness: f32,
    best_score: u32
}

impl Statistics {
    pub fn new<I>(population: &[I]) -> Self 
    where
        I: Individual
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = population[0].fitness();
        let mut sum_fitness = 0.0;
        let mut best_score: u32 = 0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
            best_score = best_score.max(individual.score());
        }

        Self {
            min_fitness,
            max_fitness,
            average_fitness: sum_fitness / (population.len() as f32),
            best_score
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.average_fitness
    }

    pub fn best_score(&self) -> u32 {
        self.best_score
    }
}