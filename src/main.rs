use std::error::Error;
use std::default::Default;
use std::time::Duration;
use std::thread;

use rustbox::{self, Color, RustBox, Key};

mod board;
mod constants;
mod snake;
mod helpers;
mod internet;

use crate::board::GameBoard;


const TICK: u64 = 1000/15;  // in milliseconds

fn main() -> Result<(), Box<dyn Error>> {
    let address = String::from("127.0.0.1:8077");
    let server = internet::Server::new(address);

    let child = thread::spawn(move || {
       server.listen();
    });

    child.join();

    return Ok(());

    let rustbox = RustBox::init(Default::default())?;

    let mut game = GameBoard::new(&rustbox);

    
    loop {
        game.update();
        game.display(&rustbox);

        match rustbox.peek_event(Duration::from_millis(TICK), false)? {
            rustbox::Event::KeyEvent(Key::Char(key)) => match key {
                'q' => { break; },
                'w' | 's' | 'a' | 'd' => game.handle_key(key),
                _ => {  },
            },
            _ => {  },
        }
    }

    Ok(())
}
