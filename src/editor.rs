#![warn(clippy::all, clippy::pedantic)]

use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initiate terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();

        if self.should_quit {
            print!("Goodbye.\r");
        } else {
            self.draw_rows();

            Terminal::cursor_position(0, 0);
        }

        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e)
}
