#![warn(clippy::all, clippy::pedantic)]

use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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
        let args: Vec<String> = env::args().collect();
        
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initiate terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = &self.cursor_position;

        let size = self.terminal.size();
        let height = u32::from(size.height.saturating_sub(1));
        let width = u32::from(size.width.saturating_sub(1));

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {VERSION}");

        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        println!("{}\r", &welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e)
}
