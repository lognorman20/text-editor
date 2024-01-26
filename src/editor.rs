use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        let mut prev_key: Key = Key::Char('_');
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }

                        // quit the program vim style
                        if c == 'q' && prev_key == Key::Char(':') {
                            break;
                        }

                        prev_key = key;
                    }

                    // handle all non ascii characters (backspace, ctrl, etc...)
                    _ => println!("{:?} goofy key\r", key),
                },
                Err(err) => {
                    die(err);
                }
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
