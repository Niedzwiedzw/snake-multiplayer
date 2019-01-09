use std::collections::VecDeque;

use crate::helpers::bound_add;
use crate::constants::{ Direction, APPLE_COUNT };
use crate::board::Coord;
use crate::board::GameBoard;

pub struct Snake {
    pub id: u8,
    pub length: usize,
    pub body: VecDeque<Coord>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(id: u8, initial: Coord) -> Self {
        let mut initial_body = VecDeque::new();
        initial_body.push_front(initial);

        Snake {
            id,
            length: 2,
            body: initial_body,
            direction: Direction::DOWN,
        }
    }

    pub fn head(&self) -> Coord { return self.body[0] }

    pub fn next_position(&self) -> Coord {
        bound_add(self.head(), self.direction.into())
    }
    pub fn update(&mut self) {
        let pos = self.next_position();
        self.body.push_front(pos);
        if self.body.len() > self.length {
            self.body.pop_back();
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if direction.opposite() != self.direction {
            self.direction = direction;
        }
    }
}
