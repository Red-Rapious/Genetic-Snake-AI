use rand::Rng;
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use crate::eye::*;

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
    apple: (u32, u32)
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            // random position for the head of the snake
            snake: vec![(rng.gen_range(0..width), rng.gen_range(0..height))],
            apple: (rng.gen_range(0..width), rng.gen_range(0..height))
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

        // Process brain

        // Move snake

        // Handle collisions
    }
}