use rand::Rng;

pub trait MutationMethod {
    /// Mutates some genes of the genome.
    fn mutate(&self, genome: &mut Vec<f32>);
}

pub struct GaussianMutation {
    chance: f64,
    coeff: f32
}

impl GaussianMutation {
    pub fn new(chance: f64, coeff: f32) -> Self {
        assert!(0.0 <= chance && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    /// Each gene has a given probability to be changed from a random value with given coefficient.
    fn mutate(&self, genome: &mut Vec<f32>) {
        let mut rng = rand::thread_rng();

        for gene in genome.iter_mut() {
            if rng.gen_bool(self.chance) {
                let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                *gene += sign * self.coeff * rng.gen::<f32>();
            } 
        }
    }
}