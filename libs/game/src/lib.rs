use rand::Rng;

pub struct Games {
    games: Vec<Game>
}

impl Games {
    pub fn new(fields: usize, width: usize, height: usize) -> Self {
        let games = (0..fields).map(|_| Game::new(width, height)).collect();

        Self { games }
    }

    pub fn games(&self) -> &Vec<Game> {
        &self.games
    }
}

pub struct Game {
    width: usize,
    height: usize,
    snake: Vec<(usize, usize)>
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            // random position for the head of the snake
            snake: vec![(rng.gen_range(0..width), rng.gen_range(0..height))]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn snake(&self) -> &Vec<(usize, usize)> {
        &self.snake
    }
}