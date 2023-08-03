use std::slice::Iter;

pub struct Eye;

impl Eye {
    pub fn new() -> Self {
        Self
    }

    pub fn process_vision(&self, snake: &Vec<(u32, u32)>, apple: (u32, u32), width: u32, height: u32) -> [f32; 8*3] {
        assert!(snake.len() >= 1);

        let mut vision = [0.0; 8*3];
        for (i, direction) in Direction::iterator().enumerate() {
            let step = direction.step();

            let mut apple_distance: Option<u32> = None;
            let wall_distance: u32;
            let mut tail_distance: Option<u32> = None;

            let mut x = snake[0].0 as i32;
            let mut y = snake[0].1 as i32;
            let mut distance = 0;

            loop {
                // Walk one step
                x += step.0;
                y += step.1;
                distance += 1;

                // Check for apple
                if apple_distance == None && x == apple.0 as i32 && y == apple.1 as i32 {
                    apple_distance = Some(distance);
                }

                // Check for tail
                let _ = snake.iter().map(|(tx, ty)| 
                    if tail_distance == None && x == *tx as i32 && y == *ty as i32 {
                        tail_distance = Some(distance);
                    }
                );

                if x + step.0 < 0 || x + step.0 >= width as i32 || y + step.1 < 0 || y + step.1 >= height as i32
                {
                    wall_distance = distance;
                    break;
                }
            }

            // Add to the vision
            vision[i+0] = match apple_distance {
                None => 0.0,
                Some(distance) => 1.0 / distance as f32
            };

            vision[i+1] = 1.0 / wall_distance as f32;

            vision[i+2] = match tail_distance {
                None => 0.0,
                Some(distance) => 1.0 / distance as f32
            };
        }
        vision
    }
}

pub enum Direction {
    Right,
    Up,
    Left,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        use self::Direction::*;
        static DIRECTIONS: [Direction; 8] = [
            Right,
            Up,
            Left,
            Down,
            UpRight,
            UpLeft,
            DownRight,
            DownLeft
            ];
        DIRECTIONS.iter()
    }

    pub fn step(&self) -> (i32, i32) {
        use self::Direction::*;
        match *self {
            Right => (1, 0),
            Up => (0, 1),
            Left => (-1, 0),
            Down => (0, -1),
            UpRight => (1, 1),
            UpLeft => (-1, 1),
            DownRight => (1, -1),
            DownLeft => (-1, -1)
        }
    }
}