use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use crate::{eye::*, snake::*};

use rand::Rng;
use nalgebra as na;

pub mod eye;
pub mod snake;

/// Gaussian Mutation chance of mutation
const MUTATION_CHANCE: f64 = 0.5;
/// Gaussian Mutation magnitude of mutation
const MUTATION_COEFF: f32 = 0.5;
/// The n-th best snakes saved by the genetic algorithm
const SAVE_BESTS: usize = 100;

/// How many steps each snake gets to live without eating apples.
const MAX_AGE: u32 = 500;
/// Everytime the snakes eats an apple, its age decreases by this amount.
const APPLE_LIFETIME_GAIN: i32 = 50;

pub struct Games {
    games: Vec<Game>,
    genetic_algorithm: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: u32,
    pub generation: usize
}

impl Games {
    /// Initialises a new simulation, with random games inside.
    pub fn new(number_games: u32, width: u32, height: u32) -> Self {
        let games = (0..number_games).map(|_| Game::new(width, height)).collect();

        Self { 
            games,
            genetic_algorithm: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::new(),
                ga::KPointsCrossover::new(1),
                ga::GaussianMutation::new(MUTATION_CHANCE, MUTATION_COEFF),
                SAVE_BESTS
            ),
            age: 0,
            generation: 0
        }
    }

    pub fn games(&mut self) -> &Vec<Game> {
        &self.games
    }

    /// Move each game that is not yet lost one tick forward. 
    /// Returns `None` if the population did not evolve, and `Some(stats)` if it did.
    pub fn step(&mut self) -> Option<ga::Statistics> {
        self.age += 1;
        let mut stats = None;
        let mut one_game_still_running = false;

        // Step each game that is not lost
        for game in self.games.iter_mut() {
            if !game.lost {
                game.step();
                one_game_still_running = true;
            }
        }

        // If every game is lost, or the countdown is over, the population evolves.
        if !one_game_still_running {
            stats = Some(self.evolve());
        }

        // If the snaked displayed on screen looses, directly trains towards the next generation
        // to avoid the feeling of a "frozen" game.
        if self.games[0].lost {
            return Some(self.train());
        }

        stats
    }

    /// Evolves the population of snakes from the games.
    fn evolve(&mut self) -> ga::Statistics {
        self.age = 0;
        self.generation += 1;

        // Convert the current Snakes to SnakeIndividuals
        let mut current_population: Vec<SnakeIndividual> = self
            .games
            .iter()
            .map(|game| SnakeIndividual::from(&game.snake))
            .collect();

        // Use the genetic algorithm to evolve the snake population
        let (evolved_population, stats) = self.genetic_algorithm.evolve(&mut current_population);

        // Replace the evolved snakes in the games
        for (game, snake_individual) in self.games.iter_mut().zip(evolved_population.iter()) {
            game.reset(Snake::from((snake_individual, game.width, game.height)));
        }

        stats
    }

    /// Fast-forward to the next generation.
    pub fn train(&mut self) -> ga::Statistics {
        loop {
            if let Some(stats) = self.step() { return stats; }
        }
    }
}

pub struct Game {
    width: u32,
    height: u32,
    apple: (u32, u32),
    pub snake: Snake,
    lost: bool,
    direction: Direction4
}

impl Game {
    /// Initialises a new grid, with a new random snake.
    pub fn new(width: u32, height: u32) -> Self {
        let snake = Snake::new(width, height);
        Self {
            width,
            height,
            // random position for the apple
            apple: Game::new_apple_position(width, height, &snake.body),
            snake: snake,
            lost: false,
            direction: Direction4::Right
        }
    }

    fn new_apple_position(width: u32, height: u32, body: &Vec<(u32, u32)>) -> (u32, u32) {
        let mut rng = rand::thread_rng();
        let mut apple = (rng.gen_range(0..width), rng.gen_range(0..height));
        while body.contains(&apple) {
            apple = (rng.gen_range(0..width), rng.gen_range(0..height));
        }
        apple
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

    pub fn apple(&self) -> (u32, u32) {
        self.apple
    }

    pub fn direction(&self) -> u8 {
        self.direction.to_u8()
    }

    /// Handles one step of the game, including snake movement, collisions handling...
    pub fn step(&mut self) {
        self.snake.age += 1;
        if self.snake.age == MAX_AGE {
            self.loose();
            return;
        }

        /* Process vision */
        let vision = self.snake.eye.process_vision(&self.snake.body, self.apple, self.width, self.height);

        /* Process brain */
        let directions_activation = self.snake.brain.feed_forward(na::DVector::from(vision.to_vec()));
        // Choose the index of the maximum activation
        let mut maxi = (0, directions_activation[0]);
        for i in 1..directions_activation.shape().0 {
            if directions_activation[i] > maxi.1 {
                maxi = (i, directions_activation[i]);
            }
        }

        let direction = Direction4::from(maxi.0);
        self.direction = direction;
        self.move_snake(direction);
    }

    /// Moves the snake and handles collisions with the walls and food.
    pub fn move_snake(&mut self, direction: Direction4) {
        let incrementer = direction.incrementer();

        // The old position of the end of the tail. 
        // Used when the tail must grown.
        let end_tail = self.snake.body[self.snake.body.len() - 1];

        // The new position of the head after movement.
        let moved_head = (self.snake.body[0].0 as i32 + incrementer.0, self.snake.body[0].1 as i32 + incrementer.1);

        // If the new head of the snake is out of the grid, the game is lost.
        if !(0 <= moved_head.0 && moved_head.0 < self.width as i32 && 0 <= moved_head.1 && moved_head.1 < self.height as i32)  {
            self.loose();
            return;
        } 

        // Move the tail: each bit takes the position of the previous bit.
        for i in (1..self.snake.body.len()).rev() {
            self.snake.body[i] = self.snake.body[i-1];
        }

        // Move the head.
        self.snake.body[0] = (moved_head.0 as u32, moved_head.1 as u32);

        /* Handle collisions with the tail */
        let (x, y) = moved_head;
        for (tx, ty) in self.snake.body.iter().skip(1) { // for each bit different of the head
            if x == *tx as i32 && y == *ty as i32 { // if head == bit
                self.loose();
                return;
            }
        }

        /* Handle collisions with the apple */
        if x == self.apple.0 as i32 && y == self.apple.1 as i32 { // if head == apple
            // Increase tail size
            self.snake.body.push(end_tail);

            // Respawn new apple
            self.apple = Game::new_apple_position(self.width, self.height, &self.snake.body);

            // Update apples_eaten
            self.snake.apples_eaten += 1;

            // Decrease age to gain more lifetime
            self.snake.age = i32::max(0, self.snake.age as i32 - APPLE_LIFETIME_GAIN) as u32;
        }
    }

    pub fn loose(&mut self) {
        self.lost = true;
    }

    /// Resets the game with a new evolved snake.
    pub fn reset(&mut self, snake: Snake) {
        // Replace old snake with new snake, and reset the `lost` boolean.
        self.snake = snake;
        self.lost = false;

        // Change apple position for visibility. This way, it is easier to spot generation changes.
        self.apple = Game::new_apple_position(self.width, self.height, &self.snake.body);
    }
}

#[derive(Clone, Copy)]
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

    pub fn to_u8(&self) -> u8 {
        use self::Direction4::*;
        match *self {
            Right => 0,
            Up => 1,
            Left => 2,
            Down => 3,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod games {
        use super::*;

        #[test]
        fn step() {
            let mut games = Games::new(100, 30, 20);
            
            for _ in 0..10 {
                games.step();
            }
        }
    }

    mod game {
        use super::*;

        #[test]
        fn loose_game() {
            let mut game = Game::new(3, 3);
            game.apple = (0,0);
            game.snake.body = vec![(1, 1)];

            game.move_snake(Direction4::Down);
            assert_eq!(game.lost, false, "Lost too quickly.");

            game.move_snake(Direction4::Down);
            assert_eq!(game.lost, true, "Did not loose whereas it should have.");
        }
    }
}