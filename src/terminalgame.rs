use std::{
    io::Write,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use termion::{
    self, clear, cursor,
    event::Event,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

struct Syncer {
    interval: Duration,
    prevtime: Instant,
}

impl Syncer {
    fn new(interval: Duration) -> Self {
        Syncer {
            interval: interval,
            prevtime: Instant::now(),
        }
    }
    fn from_secs_f64(interval: f64) -> Self {
        Self::new(Duration::from_secs_f64(interval))
    }
    fn from_fps(fps: f64) -> Self {
        Self::from_secs_f64(1f64 / fps)
    }
    fn sync(&mut self) {
        let dt = self.prevtime.elapsed();
        if self.interval - dt > Duration::new(0, 0) {
            thread::sleep(self.interval - dt);
        }
        self.prevtime += self.interval;
    }
}

pub trait TerminalGame {
    fn input(&mut self, e: Event);
    fn update(&mut self);
    fn render(&mut self, buff: &mut Vec<u8>);
    fn running(&self) -> bool;
    fn fps(&self) -> f64;
    fn start(&mut self) {
        let mut stdout = AlternateScreen::from(cursor::HideCursor::from(
            MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap()),
        ));
        let stdin = std::io::stdin();
        //
        let (tx, rx) = mpsc::channel();
        //
        thread::spawn(move || {
            for e in stdin.events() {
                if let Ok(e) = e {
                    tx.send(e).unwrap();
                }
            }
        });
        //
        writeln!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        //
        let buff = &mut Vec::new();
        let mut syncer = Syncer::from_fps(self.fps());
        //
        while self.running() {
            for e in rx.try_iter() {
                self.input(e);
            }
            //
            self.update();
            self.render(buff);
            //
            stdout.write_all(buff).unwrap();
            buff.clear();
            stdout.flush().unwrap();
            //
            syncer.sync();
        }
        writeln!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();
    }
}
