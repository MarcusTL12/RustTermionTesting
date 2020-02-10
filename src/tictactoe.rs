use std::io::Write;
use std::time::Instant;

use termion::{
    clear, cursor,
    event::{Event, Key, MouseButton, MouseEvent},
};

pub use super::terminalgame::TerminalGame;

pub struct TicTacToe {
    running: bool,
    fps: f64,
    buff: Vec<u8>,
    mouse: Option<(u16, u16, MouseButton)>,
    timer: Instant,
    ttime: f64,
    i: usize,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            running: true,
            fps: 60f64,
            buff: Vec::new(),
            mouse: None,
            timer: Instant::now(),
            ttime: 0f64,
            i: 0,
        }
    }
}

impl TerminalGame for TicTacToe {
    fn input(&mut self, e: Event) {
        match e {
            Event::Mouse(m) => match m {
                MouseEvent::Press(k, x, y) => {
                    self.mouse = Some((x, y, k));
                    match k {
                        MouseButton::Left => {
                            write!(self.buff, "{}", cursor::Goto(x, y)).unwrap()
                        }
                        MouseButton::Right => {
                            write!(self.buff, "{}", clear::All).unwrap()
                        }
                        _ => (),
                    }
                }
                MouseEvent::Hold(x, y) => {
                    write!(self.buff, "{}x", cursor::Goto(x, y)).unwrap()
                }
                MouseEvent::Release(_, _) => self.mouse = None,
            },
            Event::Key(k) => {
                if let Some((_, _, MouseButton::Right)) = self.mouse {
                } else {
                    match k {
                        Key::Char(c) => {
                            write!(self.buff, "{}", c).unwrap();
                        }
                        Key::Esc => self.running = false,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    //
    fn update(&mut self) {
        if self.i % 60 == 0 {
            self.ttime = self.timer.elapsed().as_secs_f64();
            self.i = 0;
        }
        self.i += 1;
    }
    //
    fn render(&mut self, buff: &mut Vec<u8>) {
        buff.append(&mut self.buff);
        write!(buff, "{}{}", cursor::Goto(1, 1), self.ttime).unwrap();
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
