use std::io::Write;

use termion::{
    clear, color, cursor,
    event::{Event, Key, MouseButton},
    style,
};

use termion_game_engine;
use termion_game_engine::{GameObject, TerminalGameStatic};
use termion_game_engine_util::Button;

struct TicTacToe {
    dbuff: Vec<u8>,
    running: bool,
    exitbutton: Button,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            dbuff: Vec::new(),
            running: true,
            exitbutton: Button::new((1, 1), (4, 2), color::Red),
        }
    }
}

impl TerminalGameStatic for TicTacToe {
    //
    fn update(&mut self, e: Event, buff: &mut Vec<u8>) {
        self.exitbutton.input(&e);
        match e {
            Event::Key(k) => match k {
                Key::Esc => self.running = false,
                _ => (),
            },
            _ => (),
        }
        //
        if self.exitbutton.released(MouseButton::Left) {
            self.running = false;
        }
        //
        write!(
            buff,
            "{}{}{}{}",
            clear::All,
            style::Reset,
            color::Bg(color::Reset),
            color::Fg(color::White),
        )
        .unwrap();
        //
        self.exitbutton.render(buff);
        //
        if let Ok((_, h)) = termion::terminal_size() {
            write!(buff, "{}", cursor::Goto(1, h)).unwrap();
            buff.append(&mut self.dbuff);
        }
    }
    //
    fn running(&self) -> bool {
        self.running
    }
}

fn main() {
    let mut game = TicTacToe::new();
    game.start();
}
