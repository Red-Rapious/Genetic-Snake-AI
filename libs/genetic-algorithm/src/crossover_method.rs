use rand::Rng;

pub trait CrossoverMethod {
    fn crossover(&self, parent_a: &Vec<f32>, parent_b: &Vec<f32>) -> Vec<f32>;
}

pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
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