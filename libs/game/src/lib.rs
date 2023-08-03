use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use crate::eye::*;

use rand::Rng;
use nalgebra as na;

pub mod eye;

/// Gaussian Mutation chance of mutation
const MUTATION_CHANCE: f64 = 0.01;
/// Gaussian Mutation magnitude of mutation
const MUTATION_COEFF: f32 = 0.03;

/// How many steps each snake gets to live
const GENERATION_LENGTH: u32 = 100; 

pub struct Games {
    games: Vec<Game>,
    genetic_algorithm: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: u32
}

impl Games {
    pub fn new(fields: u32, width: u32, height: u32) -> Self {
        let games = (0..fields).map(|_| Game::new(width, height)).collect();

        Self { 
            games,
            genetic_algorithm: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::new(),
                ga::UniformCrossover::new(),
                ga::GaussianMutation::new(MUTATION_CHANCE, MUTATION_COEFF)
            ),
            age: 0
        }
    }

    pub fn games(&self) -> &Vec<Game> {
        &self.games
    }

    pub fn step(&mut self) {
        self.age += 1;

        let _ = self.games.iter_mut().map(|game| game.step());

        if self.age > GENERATION_LENGTH {
            self.evolve();
        }
    }

    fn evolve(&mut self) {
        
    }
}

pub struct Game {
    width: u32,
    height: u32,
    snake: Vec<(u32, u32)>,
    apple: (u32, u32),
    eye: Eye,
    brain: nn::NeuralNetwork
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            // random position for the head of the snake
            snake: vec![(rng.gen_range(0..width), rng.gen_range(0..height))],
            apple: (rng.gen_range(0..width), rng.gen_range(0..height)),
            eye: Eye::new(),
            brain: nn::NeuralNetwork::random(&vec![8*3, 18, 18, 4], nn::ActivationFunction::sigmoid())
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn snake(&self) -> &Vec<(u32, u32)> {
        &self.snake
    }

    pub fn step(&mut self) {
        // Process vision
        let vision = self.eye.process_vision(&self.snake, self.apple, self.width, self.height);

        // Process brain
        let directions_activation = self.brain.feed_forward(na::DVector::from(vision.to_vec()));
        // Choose the index of the maximum activation
        let mut maxi = (0, 0.0);
        for i in 1..directions_activation.shape().0 {
            if directions_activation[i] > maxi.1 {
                maxi = (i, directions_activation[i]);
            }
        }
        let direction = Direction4::from(maxi.0);

        // Move snake

        // Handle collisions
    }
}

pub enum Direction4 {
    Right,
    Up,
    Left,
    Down,
}

impl From<usize> for Direction4 {
    fn from(value: usize) -> Self {
        use Direction4::*;
        match value {
            0 => Right,
            1 => Up,
            2 => Left,
            3 => Down,
            _ => panic!("Impossible direction returned by the neural network.")
        }
    }
}

impl Direction4 {
    pub fn incrementer(&self) -> (i32, i32) {
        use self::Direction4::*;
        match *self {
            Right => (1, 0),
            Up => (0, 1),
            Left => (-1, 0),
            Down => (0, -1),
        }
    }
}