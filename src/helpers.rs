use crate::constants::{WIDTH, HEIGHT};
use rustbox::{RustBox, Color};
use crate::main;
use crate::board::Coord;

pub fn bound_add(initial: (usize, usize), modifier: (i8, i8)) -> Coord {
    (
        ((WIDTH as i8 + initial.0 as i8 + modifier.0) % WIDTH as i8) as usize,
        ((HEIGHT as i8 + initial.1 as i8 + modifier.1) % HEIGHT as i8) as usize
    )
}


pub fn debug(message: String, gamecontext: &RustBox) {

    for y in 0..HEIGHT {
        gamecontext.print(0, y, rustbox::RB_BOLD, Color::Yellow, Color::Black, message.as_str());
    }
}
