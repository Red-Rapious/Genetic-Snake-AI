use std::slice::Iter;

pub struct Eye;

impl Eye {
    pub fn new() -> Self {
        Self
    }

    /// Given the position of the snake's body, the apple, and the dimensions of the grid,
    /// computes the vision of the snake.
    /// In each of the 8 directions (including diagonals), the snakes sees:
    /// - distance to apple
    /// - distance to wall
    /// - distance to tail
    pub fn process_vision(&self, body: &Vec<(u32, u32)>, apple: (u32, u32), width: u32, height: u32) -> [f32; 8*3] {
        assert!(body.len() >= 1);

        let mut vision = [0.0; 8*3];
        for (i, direction) in Direction8::iterator().enumerate() {
            let incrementer = direction.incrementer();

            // The apple and tail might not be seen, hence the Option type.
            // The wall is certain to be seen.
            let mut apple_distance: Option<u32> = None;
            let wall_distance: u32;
            let mut tail_distance: Option<u32> = None;

            let mut x = body[0].0 as i32;
            let mut y = body[0].1 as i32;
            let mut distance = 0;

            // While the wall is not seen, walk in the given direction
            loop {
                // Walk one step
                x += incrementer.0;
                y += incrementer.1;
                distance += 1;

                // Check for wall
                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32
                {
                    wall_distance = distance;
                    break;
                }

                // Check for apple
                if apple_distance == None && x == apple.0 as i32 && y == apple.1 as i32 {
                    apple_distance = Some(distance);
                }

                // Check for tail
                for (tx, ty) in body.iter() {
                    if tail_distance == None && x == *tx as i32 && y == *ty as i32 {
                        tail_distance = Some(distance);
                    }
                }
            }

            // Add to the vision array
            vision[3*i+0] = match apple_distance {
                None => 0.0,
                Some(_distance) => 1.0 // distance as f32
            };

            vision[3*i+1] = 1.0 / wall_distance as f32;

            vision[3*i+2] = match tail_distance {
                None => 0.0,
                Some(_distance) => 1.0 / distance as f32
            };
        }
        vision
    }
}

pub enum Direction8 {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl Direction8 {
    pub fn iterator() -> Iter<'static, Direction8> {
        use self::Direction8::*;
        static DIRECTIONS: [Direction8; 8] = [
            Right,
            UpRight,
            Up,
            UpLeft,
            Left,
            DownLeft,
            Down,
            DownRight,
            ];
        DIRECTIONS.iter()
    }

    pub fn incrementer(&self) -> (i32, i32) {
        use self::Direction8::*;
        match *self {
            Right => (1, 0),
            UpRight => (1, 1),
            Up => (0, 1),
            UpLeft => (-1, 1),
            Left => (-1, 0),
            DownLeft => (-1, -1),
            Down => (0, -1),
            DownRight => (1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn vision3x3() {
        let eye = Eye::new();
        let vision = eye.process_vision(&vec![(1,1), (2, 1)], (0, 0), 3, 3);

        assert_eq!(vision, 
            [//  A    W    T   // Apple, Wall, Tail
                0.0, 0.5, 1.0, // right, then counter-clockwise
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0, 
                1.0, 0.5, 0.0, // down-left, where the apple is relatively to the snake's head
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0
            ]
        );
    }

    #[test]
    pub fn vision4x4() {
        let eye = Eye::new();
        let vision = eye.process_vision(&vec![(1,1)], (3, 3), 4, 4);

        assert_eq!(vision, 
            [//  A    W    T   // Apple, Wall, Tail
                0.0, 1.0/3.0, 0.0, // right, then counter-clockwise
                1.0, 1.0/3.0, 0.0, 
                0.0, 1.0/3.0, 0.0, 
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0, // down-left, where the apple is relatively to the snake's head
                0.0, 0.5, 0.0, 
                0.0, 0.5, 0.0
            ]
        );
    }
}