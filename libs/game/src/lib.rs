use rand::Rng;

pub struct Games {
    games: Vec<Game>
}

impl Games {
    pub fn new(fields: u32, width: u32, height: u32) -> Self {
        let games = (0..fields).map(|_| Game::new(width, height)).collect();

        Self { games }
    }

    pub fn games(&self) -> &Vec<Game> {
        &self.games
    }
}

pub struct Game {
    width: u32,
    height: u32,
    snake: Vec<(u32, u32)>
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            // random position for the head of the snake
            snake: vec![(rng.gen_range(0..width), rng.gen_range(0..height))]
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
}