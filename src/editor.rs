use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    terminal: Terminal,
    should_quit: bool,
    prev_key: Key,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            should_quit: false,
            prev_key: Key::Char('_'),
        }
    }

    pub fn run(&mut self) {
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
        Terminal::clear_screen();
        if self.should_quit {
            println!("SAY GOODBYE \r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1));
        }
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.get_size().height {
            println!("~\r");
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key: Key = Terminal::read_key()?;
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

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
