use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    prev_key: Key
}

impl Editor {
    pub fn default() -> Self {
        Self{
            prev_key: Key::Char('_')
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key: Key = read_key()?;
        match pressed_key {
            Key::Char('q') => {
                if self.prev_key == Key::Char(':') {
                    panic!("Program end")
                }
            },
            _ => {
                self.prev_key = pressed_key;
            }
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
