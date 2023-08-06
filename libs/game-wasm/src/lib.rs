use gm::snake::SnakeIndividual;
use wasm_bindgen::prelude::*;
use lib_game as gm;
use lib_genetic_algorithm as ga;
use serde::Serialize;

const NUMBER_GAMES: u32 = 2_000;
const WIDTH: u32 = 30;
const HEIGHT: u32 = 30;

#[wasm_bindgen]
pub struct Games {
    games: gm::Games,
    generation: usize,
    stats: ga::Statistics
}

#[wasm_bindgen]
impl Games {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut games = gm::Games::new(NUMBER_GAMES, WIDTH, HEIGHT);
        
        let snakes_individuals: Vec<SnakeIndividual> = games.games()
            .iter()
            .map(|game| SnakeIndividual::from(&game.snake)).collect();

        Self { games, generation: 0, stats: ga::Statistics::new(&snakes_individuals) }
    }

    pub fn games(&mut self) -> JsValue {
        let games: Vec<Game> = self.games.games().iter().map(|game| Game::from(game)).collect();
        serde_wasm_bindgen::to_value(&games).unwrap()
    }

    pub fn step(&mut self) {
        if let Some(stats) = self.games.step() { 
            self.generation += 1; 
            self.stats = stats;
        }
    }

    pub fn train(&mut self) {
        self.games.train();
        self.generation += 1;
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn min_fitness(&self) -> usize {
        self.stats.min_fitness() as usize
    }

    pub fn max_fitness(&self) -> usize {
        self.stats.max_fitness() as usize
    }

    pub fn avg_fitness(&self) -> usize {
        self.stats.avg_fitness() as usize
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Game {
    width: u32,
    height: u32,
    snake: Vec<(u32, u32)>,
    apple: (u32, u32)
}

impl Game {
    pub fn snake(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.snake).unwrap()
    }

    pub fn apple(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.apple).unwrap()
    }
}

impl From<&gm::Game> for Game {
    fn from(game: &gm::Game) -> Self {
        Self { width: game.width(), height: game.height(), snake: game.body().clone(), apple: game.apple() }
    }
}