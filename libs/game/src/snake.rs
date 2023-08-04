use crate::*;

const APPLES_COEFF: u32 = 20;
const AGE_COEFF: u32 = 1;

pub struct Snake {
    pub(crate) body: Vec<(u32, u32)>,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::NeuralNetwork,
    pub(crate) age: u32,
    pub(crate) apples_eaten: u32
}

impl Snake {
    pub fn new(/*width: u32, height: u32*/) -> Self {
        //let mut rng = rand::thread_rng();

        Self {
            body: vec![(0, 0)],//vec![(rng.gen_range(0..width), rng.gen_range(0..height))],
            eye: Eye::new(),
            brain: nn::NeuralNetwork::random(),
            age: 0,
            apples_eaten: 0
        }
    }
}

pub struct SnakeIndividual {
    apples_eaten: u32,
    age: u32,
    genome: Vec<f32>,
}

impl From<&Snake> for SnakeIndividual {
    fn from(snake: &Snake) -> Self {
        Self { 
            apples_eaten: snake.apples_eaten, 
            age: snake.age, 
            genome: snake.brain.to_weights(),
        }
    }
}

impl From<&SnakeIndividual> for Snake {
    fn from(snake_individual: &SnakeIndividual) -> Self {
        Self { 
            body: vec![(0, 0)], 
            eye: Eye::new(), 
            brain: nn::NeuralNetwork::from_weights(&snake_individual.genome), 
            age: 0, 
            apples_eaten: 0 
        }
    }
}

impl ga::Individual for SnakeIndividual {
    fn fitness(&self) -> f32 {
        (self.apples_eaten * APPLES_COEFF + self.age * AGE_COEFF) as f32
    }

    // Used inside the GeneticAlgorithm to convert an evolved chromosome back to an Individual
    fn create(genom: Vec<f32>) -> Self {
        Self {
            apples_eaten: 0, // Useless values, except for genom. Might need to change that for clarity.
            age: 0,
            genome: genom
        }
    }

    fn genome(&self) -> &Vec<f32> {
        &self.genome
    }
}