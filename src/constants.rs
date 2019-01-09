pub const WIDTH: usize = 50;
pub const HEIGHT: usize = WIDTH;
pub const APPLE_COUNT: usize = 2;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Direction {
    pub fn num(&self) -> (i8, i8) {
        match self {
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::UP => (0, -1),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

impl Into<(i8, i8)> for Direction {
    fn into(self) -> (i8, i8) {
        self.num()
    }
}
