use std::io::Write;

use termion::{
    clear, color, cursor,
    event::{Event, Key, MouseButton},
    style,
};

use termion_game_engine::{GameObject, TerminalGameStatic};
use termion_game_engine_util::{box_mix, Button, TextLabel};

struct Board {
    _board: Vec<Vec<Option<bool>>>,
    _turn: bool,
    pos: (u16, u16),
    n: usize,
}

impl Board {
    fn new(pos: (u16, u16), n: usize) -> Self {
        Board {
            _board: (0..n).map(|_| (0..n).map(|_| None).collect()).collect(),
            _turn: false,
            pos: pos,
            n: n,
        }
    }
}

impl GameObject for Board {
    fn input(&mut self, _e: &Event) {}
    //
    // fn render(&mut self, buff: &mut Vec<u8>) {
    //     write!(
    //         buff,
    //         "{}{}",
    //         cursor::Goto(self.pos.0, self.pos.1),
    //         color::Fg(color::Reset)
    //     )
    //     .unwrap();
    //     for y in 0..self.n {
    //         let top = y == 0;
    //         let upind = !top as usize * 3 * 3 + 3 * 3 * 3;
    //         for x in 0..self.n {
    //             let left = x == 0;
    //             let ind = !left as usize * 1 * 3 + 1 + upind;
    //             write!(buff, "{}", BOX_MIX[ind]).unwrap();
    //             for _ in 0..3 {
    //                 write!(buff, "{}", BOX_MIX[1 + 3]).unwrap();
    //             }
    //         }
    //         write!(buff, "{}", BOX_MIX[3 + upind]).unwrap();
    //         write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))
    //             .unwrap();
    //         for _ in 0..self.n {
    //             write!(buff, "{}", BOX_MIX[3 * 3 + 3 * 3 * 3]).unwrap();
    //             write!(buff, " {} ", ' ').unwrap();
    //         }
    //         write!(buff, "{}", BOX_MIX[3 * 3 + 3 * 3 * 3]).unwrap();
    //         write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))
    //             .unwrap();
    //     }
    //     for x in 0..self.n {
    //         let left = x == 0;
    //         let ind = !left as usize * 1 * 3 + 1 + 3 * 3;
    //         write!(buff, "{}", BOX_MIX[ind]).unwrap();
    //         for _ in 0..3 {
    //             write!(buff, "{}", BOX_MIX[1 + 3]).unwrap();
    //         }
    //     }
    //     write!(buff, "{}", BOX_MIX[3 + 3 * 3]).unwrap();
    // }
    fn render(&mut self, buff: &mut Vec<u8>) {
        write!(
            buff,
            "{}{}",
            cursor::Goto(self.pos.0, self.pos.1),
            color::Fg(color::Reset)
        )
        .unwrap();
        //
        for y in 0..self.n {
            let top = y == 0;
            let bottom = y == self.n + 1;
            for x in 0..self.n {
                let left = x == 0;
                // let right = x == self.n + 1;
                write!(
                    buff,
                    "{}",
                    box_mix([
                        Some(top),
                        if !left { Some(top) } else { None },
                        if !top { Some(left) } else { None },
                        Some(left)
                    ])
                )
                .unwrap();
                for _ in 0..3 {
                    write!(
                        buff,
                        "{}",
                        box_mix([Some(top), Some(top), None, None])
                    )
                    .unwrap();
                }
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
            )
            .unwrap();
            write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))
                .unwrap();
            for _ in 0..self.n {
                write!(buff, "{}", box_mix([None, None, None, None])).unwrap();
                write!(buff, " {} ", ' ').unwrap();
            }
            write!(buff, "{}", box_mix([None, None, Some(true), Some(true)]))
                .unwrap();
            write!(buff, "\n{}", cursor::Left((self.n * 4 + 1) as u16))
                .unwrap();
        }
    }
}

struct TicTacToe {
    dbuff: Vec<u8>,
    running: bool,
    exitbutton: Button,
    exitlabel: TextLabel,
    board: Board,
    temp: usize,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            dbuff: Vec::new(),
            running: true,
            exitbutton: Button::new((1, 1), (4, 2), color::Red),
            exitlabel: TextLabel::new(
                (5, 2),
                String::from("<- Exit"),
                color::Red,
            ),
            board: Board::new((20, 10), 3),
            temp: 0,
        }
    }
}

impl TerminalGameStatic for TicTacToe {
    fn update(&mut self, e: Event, buff: &mut Vec<u8>) {
        self.exitbutton.input(&e);
        self.board.input(&e);
        //
        match e {
            Event::Key(Key::Char(' ')) => self.temp += 1,
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
        self.exitlabel.render(buff);
        self.board.render(buff);
        //
        write!(
            buff,
            "{}{}{}",
            cursor::Goto(1, 20),
            color::Fg(color::White),
            // BOX_MIX[self.temp % BOX_MIX.len()]
            box_mix([Some(true), Some(false), None, Some(true)])
        )
        .unwrap();
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
