use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use crate::{eye::*, snake::*};

use rand::Rng;
use nalgebra as na;

pub mod eye;
pub mod snake;

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
    apple: (u32, u32),
    snake: Snake,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            // random position for the head of the snake
            apple: (rng.gen_range(0..width), rng.gen_range(0..height)),
            snake: Snake::new()//(width, height)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn body(&self) -> &Vec<(u32, u32)> {
        &self.snake.body
    }

    pub fn step(&mut self) {
        /* Process vision */
        let vision = self.snake.eye.process_vision(&self.snake.body, self.apple, self.width, self.height);

        /* Process brain */
        let directions_activation = self.snake.brain.feed_forward(na::DVector::from(vision.to_vec()));
        // Choose the index of the maximum activation
        let mut maxi = (0, 0.0);
        for i in 1..directions_activation.shape().0 {
            if directions_activation[i] > maxi.1 {
                maxi = (i, directions_activation[i]);
            }
        }
        let direction = Direction4::from(maxi.0);

        /* Move snake */
        let incrementer = direction.incrementer();

        let moved_head = (self.snake.body[0].0 as i32 + incrementer.0, self.snake.body[0].1 as i32 + incrementer.1);
        let end_tail = self.snake.body[self.snake.body.len() - 1];

        // Impossible position
        if !(0 <= moved_head.0 && moved_head.0 < self.width as i32 && 0 <= moved_head.1 && moved_head.1 < self.height as i32)  {
            self.loose();
        } 


        // Move the tail: each bit takes the position of the previous bit
        for i in (1..self.snake.body.len()).rev() {
            self.snake.body[i] = self.snake.body[i-1];
        }

        // Move the head
        self.snake.body[0] = (moved_head.0 as u32, moved_head.1 as u32);

        /* Handle collisions with the tail */
        let (x, y) = moved_head;
        for (tx, ty) in self.snake.body.iter() {
            if x == *tx as i32 && y == *ty as i32 {
                self.loose();
                break;
            }
        }

        /* Handle collisions with the apple */
        if x == self.apple.0 as i32 && y == self.apple.1 as i32 {
            todo!();
        }
    }

    pub fn loose(&self) {
        todo!();
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