use crate::*;

const APPLES_COEFF: u32 = 20;
const AGE_COEFF: u32 = 1;

pub struct Snake {
    pub(crate) body: Vec<(u32, u32)>,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::NeuralNetwork,
    age: Option<u32>,
    apples_eaten: u32
}

impl Snake {
    pub fn new(/*width: u32, height: u32*/) -> Self {
        //let mut rng = rand::thread_rng();

        Self {
            body: vec![(0, 0)],//vec![(rng.gen_range(0..width), rng.gen_range(0..height))],
            eye: Eye::new(),
            brain: nn::NeuralNetwork::random(&vec![8*3, 18, 18, 4], nn::ActivationFunction::sigmoid()),
            age: None,
            apples_eaten: 0
        }
    }
}

impl ga::Individual for Snake {
    fn fitness(&self) -> f32 {
        (self.apples_eaten * APPLES_COEFF + self.age.unwrap() * AGE_COEFF) as f32
    }

    fn create(genom: Vec<f32>) -> Self {
        let mut snake = Snake::new();
        snake.brain = nn::NeuralNetwork::from_weights(genom);

        snake
    }

    fn genome(&self) -> Vec<f32> {
        self.brain.to_weights()
    }
}