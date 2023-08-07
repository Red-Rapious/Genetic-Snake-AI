use rand::Rng;
use itertools::Itertools;

pub trait CrossoverMethod {
    /// Given the genomes of two parents, creates the genome of a child.
    fn crossover(&self, parent_a: &Vec<f32>, parent_b: &Vec<f32>) -> Vec<f32>;
}


pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    /// Each gene of the genome has a 0.5 probability to come from each parent,
    /// and is independant from other genes.
    fn crossover(&self, parent_a: &Vec<f32>, parent_b: &Vec<f32>) -> Vec<f32> {
        assert_eq!(parent_a.len(), parent_b.len());

        let mut rng = rand::thread_rng();

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&gene_a, &gene_b)| if rng.gen_bool(0.5) { gene_a } else { gene_b })
            .collect()
    }
}


pub struct KPointsCrossover {
    k: usize
}

impl KPointsCrossover {
    pub fn new(k: usize) -> Self {
        Self { k }
    }
}

impl CrossoverMethod for KPointsCrossover {
    fn crossover(&self, parent_a: &Vec<f32>, parent_b: &Vec<f32>) -> Vec<f32> {
        assert_eq!(parent_a.len(), parent_b.len());
        
        let mut rng = rand::thread_rng();
        let mut points: Vec<usize> = (0..self.k)
            .map(|_| rng.gen_range(0..parent_a.len()))
            .unique()
            .collect();
        points.sort_unstable();

        let mut flip = true;
        let mut cursor = 0;

        parent_a
            .iter()
            .enumerate()
            .zip(parent_b.iter())
            .map(|((i, &gene_a), &gene_b)| {
                if cursor != self.k && i == points[cursor] {
                    flip = !flip;
                    cursor += 1;
                }
                if flip { 
                    gene_a 
                } else { 
                    gene_b 
                }})
            .collect()
    }
}