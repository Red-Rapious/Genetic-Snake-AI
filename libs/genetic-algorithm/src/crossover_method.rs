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
        assert!(k > 0);

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
                if cursor != points.len() && i == points[cursor] {
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


pub struct IntermediateCrossover {
    alpha: f32
}

impl IntermediateCrossover {
    pub fn new(alpha: f32) -> Self {
        assert!(0.0 < alpha && alpha < 1.0);

        Self { alpha }
    }
} 

impl CrossoverMethod for IntermediateCrossover {
    fn crossover(&self, parent_a: &Vec<f32>, parent_b: &Vec<f32>) -> Vec<f32> {
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&gene_a, &gene_b)| {
                gene_a * (1.0 - self.alpha) + gene_b * self.alpha
            }
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod k_points_crossover {
        use super::*;

        #[test]
        fn test_k1() {
            let kpc = KPointsCrossover::new(1);
            let vect = (0..1_000).map(|i| i as f32).collect();

            for _ in 0..10_000 {
                kpc.crossover(&vect, &vect);
            }
        }

        #[test]
        fn test_k2() {
            let kpc = KPointsCrossover::new(2);
            let vect = (0..1_000).map(|i| i as f32).collect();
            
            for _ in 0..10_000 {
                kpc.crossover(&vect, &vect);
            }
        }
    }
}