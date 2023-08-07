use crate::*;

//const APPLES_COEFF: u32 = 100;
//const AGE_COEFF: u32 = 1;

pub struct Snake {
    pub(crate) body: Vec<(u32, u32)>,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::NeuralNetwork,
    pub(crate) age: u32,
    pub(crate) apples_eaten: u32
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width >= 3);
        //let mut rng = rand::thread_rng();
        let x = width/2;//rng.gen_range(2..width);
        let y = height/2;//rng.gen_range(0..height);
        let body = vec![
            (x+2, y),
            (x+1, y),
            (x, y)
        ];

        Self {
            body,
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
            genome: snake.brain.to_genome(),
        }
    }
}

impl From<(&SnakeIndividual, u32, u32)> for Snake {
    fn from((snake_individual, width, height): (&SnakeIndividual, u32, u32)) -> Self {
        let mut snake = Snake::new(width, height);
        snake.brain = nn::NeuralNetwork::from_genome(&snake_individual.genome);
        snake
    }
}

impl ga::Individual for SnakeIndividual {
    fn fitness(&self) -> f32 {
        //(self.apples_eaten * APPLES_COEFF + self.age * AGE_COEFF) as f32

        if self.apples_eaten < 10 {
            (self.age * self.age) as f32 * 2.0_f32.powf(self.apples_eaten as f32)
        } else {
            (self.age * self.age) as f32 * 2.0_f32.powf(10.0) * (self.apples_eaten - 9) as f32
        }

       //i32::max(self.age as i32 - 15, 0) as f32
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

    fn score(&self) -> u32 {
        self.apples_eaten
    }
}