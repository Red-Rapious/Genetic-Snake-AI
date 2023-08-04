use wasm_bindgen::prelude::*;
use lib_game as gm;
use serde::Serialize;

#[wasm_bindgen]
pub struct Games {
    games: gm::Games,
}

#[wasm_bindgen]
impl Games {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let games = gm::Games::new(4, 30, 20);

        Self { games }
    }

    pub fn games(&mut self) -> JsValue {
        let games: Vec<Game> = self.games.games().iter().map(|game| Game::from(game)).collect();
        serde_wasm_bindgen::to_value(&games).unwrap()
    }

    pub fn step(&mut self) {
        self.games.step();
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Game {
    width: u32,
    height: u32,
    snake: Vec<(u32, u32)>
}

impl Game {
    pub fn snake(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.snake).unwrap()
    }
}

impl From<&gm::Game> for Game {
    fn from(game: &gm::Game) -> Self {
        Self { width: game.width(), height: game.height(), snake: game.body().clone() }
    }
}