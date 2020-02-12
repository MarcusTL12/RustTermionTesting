use std::io::Write;
use std::time::Instant;


use termion::{
    clear, color, cursor,
    event::{Event, Key, MouseButton},
    style,
};

use termion_game_engine::{EveryNSync, GameObject, TerminalGame};
use termion_game_engine_util::Button;

struct TicTacToe {
    dbuff: Vec<u8>,
    running: bool,
    fps: f64,
    timer: Instant,
    ttime: f32,
    showtimer: EveryNSync,
    exitbutton: Button,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            dbuff: Vec::new(),
            running: true,
            fps: 60f64,
            timer: Instant::now(),
            ttime: 0f32,
            showtimer: EveryNSync::from_secs_f64(1f64),
            exitbutton: Button::new((1, 1), (4, 2), color::Red),
        }
    }
}

impl TerminalGame for TicTacToe {
    fn init(&mut self) {}
    //
    fn input(&mut self, e: Event) {
        self.exitbutton.input(&e);
        match e {
            Event::Key(k) => match k {
                Key::Esc => self.running = false,
                _ => (),
            },
            _ => (),
        }
    }
    //
    fn update(&mut self) {
        if self.showtimer.run() {
            self.ttime = self.timer.elapsed().as_secs_f32();
        }
        if self.exitbutton.released(MouseButton::Left) {
            self.running = false;
        }
    }
    //
    fn render(&mut self, buff: &mut Vec<u8>) {
        write!(
            buff,
            "{}{}{}{}",
            clear::All,
            style::Reset,
            color::Bg(color::Reset),
            color::Fg(color::White),
        )
        .unwrap();
        self.exitbutton.render(buff);
        if let Ok((_, h)) = termion::terminal_size() {
            write!(
                buff,
                "{}Seconds played: {}",
                cursor::Goto(1, h - 1),
                self.ttime
            )
            .unwrap();
            write!(buff, "{}", cursor::Goto(1, h)).unwrap();
            buff.append(&mut self.dbuff);
        }
    }
    //
    fn running(&self) -> bool {
        self.running
    }
    //
    fn fps(&self) -> f64 {
        self.fps
    }
}

fn main() {
    let mut game = TicTacToe::new();
    game.start();
}
