use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    prev_key: Key,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            prev_key: Key::Char('_'),
        }
    }

    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        if self.should_quit {
            println!("SAY GOODBYE \r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1));
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key: Key = read_key()?;
        match pressed_key {
            Key::Char('q') => {
                if self.prev_key == Key::Char(':') {
                    self.should_quit = true;
                }
            }
            _ => {
                self.prev_key = pressed_key;
                ()
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
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
