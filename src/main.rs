// use std::;

// use terminalgraphics::{
//     self,
//     color::{self, Color},
//     linereset::showandreset,
// };

use std::{
    io::Write,
    sync::{mpsc, Arc, Mutex},
    thread, time,
};

use termion::{
    self, clear, cursor,
    event::{Event, Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

fn main() {
    let mut stdout = AlternateScreen::from(termion::cursor::HideCursor::from(
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
    let done = Arc::new(Mutex::new(false));
    let (buff_tx, buff_rx) = mpsc::channel();
    //
    {
        let done = Arc::clone(&done);
        thread::spawn(move || {
            let mut buff = Vec::new();
            for e in rx.iter() {
                let mut done = done.lock().unwrap();
                match e {
                    Event::Mouse(me) => match me {
                        MouseEvent::Press(_, x, y) => {
                            writeln!(&mut buff, "{}x", cursor::Goto(x, y))
                                .unwrap()
                        }
                        _ => (),
                    },
                    Event::Key(k) => match k {
                        Key::Esc => {
                            *done = true;
                            break;
                        }
                        Key::Char(c) => {
                            writeln!(
                                &mut buff,
                                "{}{}",
                                // clear::All,
                                cursor::Goto(1, 2),
                                c
                            )
                            .unwrap();
                        }
                        _ => (),
                    },
                    _ => (),
                }
                buff.iter().cloned().for_each(|x| buff_tx.send(x).unwrap());
                buff.clear();
            }
            writeln!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
            stdout.flush().unwrap();
        });
    }
    //
    while !*done.lock().unwrap() {
        // let mut buff = buff.lock().unwrap();
        // stdout.write_all(&*buff).unwrap();
        // (*buff).clear();
        thread::sleep(time::Duration::from_millis(50));
    }
    //
    // for e in rx.iter() {
    //     match e {
    //         Event::Mouse(me) => match me {
    //             MouseEvent::Press(_, x, y) => {
    //                 writeln!(&mut buff, "{}x", cursor::Goto(x, y)).unwrap()
    //             }
    //             _ => (),
    //         },
    //         Event::Key(k) => match k {
    //             Key::Esc => break,
    //             Key::Char(c) => {
    //                 writeln!(
    //                     &mut buff,
    //                     "{}{}",
    //                     // clear::All,
    //                     cursor::Goto(1, 2),
    //                     c
    //                 )
    //                 .unwrap();
    //             }
    //             _ => (),
    //         },
    //         _ => (),
    //     }
    //     stdout.write_all(&buff).unwrap();
    //     buff.clear();
    // }
}
