use std::{error::Error, io::Write, iter::once};

use termion::{
    clear, color, cursor,
    event::{Event, Key, MouseButton, MouseEvent},
    style,
};

use termion_game_engine::{GameObject, TerminalGameStatic};
use termion_game_engine_util::{box_mix, Button, TextLabel};

struct Board {
    board: Vec<Vec<Option<bool>>>,
    turn: bool,
    pos: (u16, u16),
    n: usize,
    cols: [Vec<u8>; 2],
    won: Option<bool>,
    turns: usize,
}

impl Board {
    fn new(pos: (u16, u16), n: usize) -> Result<Self, Box<dyn Error>> {
        Ok(Board {
            board: (0..n).map(|_| (0..n).map(|_| None).collect()).collect(),
            turn: false,
            pos: pos,
            n: n,
            cols: [
                termion_game_engine::col2fg_str(color::Red)?,
                termion_game_engine::col2fg_str(color::Green)?,
            ],
            won: None,
            turns: 0,
        })
    }
    //
    fn reset(&mut self) {
        for y in 0..self.n {
            for x in 0..self.n {
                self.board[y][x] = None;
            }
        }
        self.turn = false;
        self.won = None;
        self.turns = 0;
    }
    //
    fn getcell(&self, mpos: (u16, u16)) -> Option<(u16, u16)> {
        let x = mpos.0 as i16 - self.pos.0 as i16;
        let y = mpos.1 as i16 - self.pos.1 as i16;
        if x < 0
            || x >= (self.n * 4) as i16
            || y < 0
            || y >= (self.n * 2) as i16
            || x % 4 == 0
            || y % 2 == 0
        {
            None
        } else {
            Some((x as u16 / 4, y as u16 / 2))
        }
    }
    //
    fn winner(&mut self) -> Option<bool> {
        let temp = (0..self.n)
            .map(|y| {
                (0..self.n).map(move |x| (x, y)).fold(
                    (0, 0),
                    |(mut a, mut b), (x, y)| {
                        if let Some(v) = self.board[y][x] {
                            a += v as i32 * 2 - 1;
                        }
                        if let Some(v) = self.board[x][y] {
                            b += v as i32 * 2 - 1;
                        }
                        (a, b)
                    },
                )
            })
            .chain(once((0..self.n).fold((0, 0), |(mut a, mut b), i| {
                if let Some(v) = self.board[i][i] {
                    a += v as i32 * 2 - 1;
                }
                if let Some(v) = self.board[i][self.n - i - 1] {
                    b += v as i32 * 2 - 1;
                }
                (a, b)
            })))
            .map(|(a, b)| once(a).chain(once(b)))
            .flatten()
            .find(|v| v.abs() as usize == self.n);
        if let Some(t) = temp {
            Some(t < 0)
        } else {
            None
        }
    }
}

impl GameObject for Board {
    fn input(&mut self, e: &Event) {
        match e {
            Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
                if let Some(cell) = self.getcell((*x, *y)) {
                    if self.won.is_none()
                        && self.board[cell.1 as usize][cell.0 as usize]
                            .is_none()
                    {
                        self.board[cell.1 as usize][cell.0 as usize] =
                            Some(self.turn);
                        self.turn = !self.turn;
                        self.turns += 1;
                        self.won = self.winner();
                    }
                }
            }
            _ => (),
        }
    }
    //
    fn render(&mut self, buff: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        if self.turns == self.n * self.n {
            write!(
                buff,
                "{}{}{}TIE!{}",
                cursor::Goto(self.pos.0, self.pos.1 - 1),
                color::Fg(color::Reset),
                style::Bold,
                style::Reset
            )?;
        } else if let Some(winner) = self.won {
            buff.extend(self.cols[!winner as usize].iter());
            write!(
                buff,
                "{}{}{}{}{} won!",
                cursor::Goto(self.pos.0, self.pos.1 - 1),
                style::Bold,
                if winner { 'X' } else { 'O' },
                color::Fg(color::Reset),
                style::Reset,
            )?;
        } else {
            write!(
                buff,
                "{}{}Turn: ",
                color::Fg(color::Reset),
                cursor::Goto(self.pos.0, self.pos.1 - 1)
            )?;
            buff.extend(self.cols[self.turn as usize].iter());
            write!(
                buff,
                "{}{}{}{}",
                style::Bold,
                if self.turn { 'O' } else { 'X' },
                color::Fg(color::Reset),
                style::Reset,
            )?;
        }
        write!(buff, "{}", cursor::Goto(self.pos.0, self.pos.1))?;
        //
        for y in 0..self.n {
            let top = y == 0;
            let bottom = y == self.n + 1;
            for x in 0..self.n {
                let left = x == 0;
                write!(
                    buff,
                    "{}",
                    box_mix([
                        Some(top),
                        if !left { Some(top) } else { None },
                        if !top { Some(left) } else { None },
                        Some(left)
                    ])
                )?;
                write!(
                    buff,
                    "{0}{0}{0}",
                    box_mix([Some(top), Some(top), None, None])
                )?;
            }
            write!(
                buff,
                "{}",
                box_mix([
                    None,
                    Some(top),
                    if !top { Some(true) } else { None },
                    Some(!bottom)
                ])
            )?;
            write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))?;
            for x in 0..self.n {
                let left = x == 0;
                write!(
                    buff,
                    "{}",
                    box_mix([None, None, Some(left), Some(left)])
                )?;
                if let Some(v) = self.board[y][x] {
                    buff.extend(self.cols[v as usize].iter());
                }
                write!(
                    buff,
                    " {}{}{}{} ",
                    style::Bold,
                    if let Some(v) = self.board[y][x] {
                        ['X', 'O'][v as usize]
                    } else {
                        ' '
                    },
                    style::Reset,
                    color::Fg(color::Reset)
                )?;
            }
            write!(buff, "{}", box_mix([None, None, Some(true), Some(true)]))?;
            write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))?;
        }
        for x in 0..self.n {
            let left = x == 0;
            write!(
                buff,
                "{}",
                box_mix([
                    Some(true),
                    if !left { Some(true) } else { None },
                    Some(left),
                    None
                ])
            )?;
            write!(
                buff,
                "{0}{0}{0}",
                box_mix([Some(true), Some(true), None, None])
            )?;
        }
        write!(buff, "{}", box_mix([None, Some(true), Some(true), None]))?;
        Ok(())
    }
}

struct TicTacToe {
    dbuff: Vec<u8>,
    running: bool,
    exitbutton: Button,
    resetbutton: Button,
    exitlabel: TextLabel,
    resetlabel: TextLabel,
    board: Board,
}

impl TicTacToe {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(TicTacToe {
            dbuff: Vec::new(),
            running: true,
            exitbutton: Button::new((1, 1), (4, 2), color::Red)?,
            resetbutton: Button::new((15, 1), (4, 2), color::Green)?,
            exitlabel: TextLabel::new(
                (6, 2),
                String::from("<-  Exit"),
                color::Red,
            )?,
            resetlabel: TextLabel::new(
                (6, 1),
                String::from("Reset ->"),
                color::Green,
            )?,
            board: Board::new((1, 5), 3)?,
        })
    }
}

impl TerminalGameStatic for TicTacToe {
    fn update(
        &mut self,
        e: Event,
        buff: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        self.exitbutton.input(&e);
        self.resetbutton.input(&e);
        self.board.input(&e);
        //
        match e {
            Event::Key(Key::Esc) => self.running = false,
            _ => (),
        }
        //
        if self.exitbutton.released(MouseButton::Left) {
            self.running = false;
        }
        //
        if self.resetbutton.released(MouseButton::Left) {
            self.board.reset();
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
        self.exitbutton.render(buff)?;
        self.resetbutton.render(buff)?;
        self.exitlabel.render(buff)?;
        self.resetlabel.render(buff)?;
        self.board.render(buff)?;
        //
        if let Ok((_, h)) = termion::terminal_size() {
            write!(buff, "{}", cursor::Goto(1, h)).unwrap();
            buff.append(&mut self.dbuff);
        }
        Ok(())
    }
    //
    fn running(&self) -> bool {
        self.running
    }
}

fn main() {
    let mut game = TicTacToe::new().expect("Game to load");
    game.start().expect("Game to start");
}
