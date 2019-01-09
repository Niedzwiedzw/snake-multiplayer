//use std::iter::Zip;

use rustbox::{self, RustBox, Color};
use rand::{self, Rng};


use crate::constants::{WIDTH, HEIGHT, Direction::{UP, DOWN, LEFT, RIGHT}};
use crate::snake::Snake;
use crate::helpers::debug;
use crate::constants::APPLE_COUNT;



#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Snake(u8),
    Apple,
    Empty,
}

pub type Coord = (usize, usize);

pub struct GameBoard<'context> {
    board: [[Cell; WIDTH]; HEIGHT],
    snakes: Vec<Snake>,
    context: &'context RustBox,
    apples: Vec<Coord>,
}

impl<'board, 'context: 'board> GameBoard<'board> {
    pub fn new(context: &'context RustBox) -> Self {
        GameBoard {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            snakes: vec![Snake::new(0, (10, 10))],
            context,
            apples: vec![],
        }
    }

    pub fn clear(&mut self) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::Empty
            }
        }
    }

    fn random_spot(&self) -> Coord {
        let x = rand::thread_rng().gen_range(0, WIDTH);
        let y = rand::thread_rng().gen_range(0, HEIGHT);

        (x, y)
    }

    fn occupied_spots(&self) -> Vec<Coord> {
        let mut coords = vec![];

        for snake in self.snakes.iter() {
            for coord in &snake.body {
                coords.push(*coord)
            }
        }

        for coord in &self.apples {
            coords.push(*coord)
        }

        coords
    }

    pub fn spawn_apple(&mut self) {
        self.apples.push(self.empty_spot());
    }

    pub fn empty_spot(&self) -> Coord {
        loop {
            let spot = self.random_spot();
            for bad_spot in self.occupied_spots() {
                if spot == bad_spot { continue; }
            }
            return spot
        }
    }

    pub fn check_collision(&mut self) {
        // apples
        let mut to_remove = vec![];
        for snake in self.snakes.iter_mut() {
            let next = &snake.next_position();

            for (index, apple) in self.apples.iter().enumerate() {
                if apple == next {
                    &to_remove.push(index);
                    snake.length += 1;
                }
            }
        }

        for index in to_remove.iter().rev() {
            self.apples.remove(*index);
        }

        // snakes
        let mut to_remove = vec![];
        for snake in self.snakes.iter() {
            let next = &snake.next_position();
            for (index, other_snake) in self.snakes.iter().enumerate() {
                for cell in other_snake.body.iter() {
                    if next == cell {
                        to_remove.push(index);
                    }
                }
            }
        }

        for index in to_remove.iter().rev() {
            self.snakes.remove(*index);
        }

        if self.snakes.len() < 1 {
            panic!("Game over!");
        }
    }

    pub fn display(&mut self, gamecontext: &RustBox) {
        self.clear();
        for snake in self.snakes.iter() {
            for (x, y) in &snake.body {
                self.board[*y][*x] = Cell::Snake(snake.id)
            }
        }

        for (x, y) in self.apples.iter() {
            self.board[*y][*x] = Cell::Apple;
        }

        for (y, row) in self.board.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let repr = match element {
                    Cell::Snake(_id) => "# ",
                    Cell::Apple => "@ ",
                    Cell::Empty => "  ",
                };

                gamecontext.print(x*2, y, rustbox::RB_BOLD, Color::White, Color::Black, repr);
            }
        }

        gamecontext.present();
    }

    pub fn handle_key(&mut self, key: char) {
        let snake = &mut self.snakes[0];
        let direction = match key {
            'w' => UP,
            's' => DOWN,
            'a' => LEFT,
            _ => RIGHT,
        };

        snake.turn(direction);
    }

    pub fn debug(&self, message: String) {
        debug(message, self.context);
    }

    pub fn update(&mut self) {
        self.check_collision();
        for snake in self.snakes.iter_mut() {
            snake.update();
        }

        if self.apples.len() < APPLE_COUNT {
            self.spawn_apple();
        }
    }
}
