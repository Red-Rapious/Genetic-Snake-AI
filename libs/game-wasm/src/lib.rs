use wasm_bindgen::prelude::*;
//use lib_game as gm;
//use serde::Serialize;

#[wasm_bindgen]
pub struct Games;

#[wasm_bindgen]
impl Games {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }
}