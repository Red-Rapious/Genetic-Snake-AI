use crate::*;

// Coefficients used if the fitness is a linear combinaison of both.
const APPLES_COEFF: u32 = 100;
//const AGE_COEFF: u32 = 1;

/// A struct holding the body, brain, and eyes of the snake.
pub struct Snake {
    pub(crate) body: Vec<(u32, u32)>,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::NeuralNetwork,
    pub(crate) age: u32,
    pub(crate) apples_eaten: u32
}

impl Snake {
    /// Creates a new Snake, in the middle of the grid, with a random brain.
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width >= 3);

        let x = width/2;
        let y = height/2;
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

#[derive(Clone)]
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
    fn fitness(&self) -> u32 {
        //(self.apples_eaten * APPLES_COEFF + self.age * AGE_COEFF)

        //if self.apples_eaten < 10 {
            (self.age * self.age) * 2_u32.pow(self.apples_eaten) * (APPLES_COEFF * self.apples_eaten + 1)
        //} else {
        //    (self.age * self.age) * 2_u32.pow(10) * (self.apples_eaten - 9) * (APPLES_COEFF * 10)
        //}
    }

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