use crate::Terminal;
use std::io::Error;
use termion::event::Key;

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit: bool,
    cursor_position: Position,
    terminal: Terminal,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            quit: false,
            cursor_position: Position::default(),
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            //self.refresh_screen();
            if self.quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }
        println!("Press\r");
        Ok(())
    }

    fn refresh_screen(&self) {
        Terminal::cursor_position(&self.cursor_position);
    }
}

fn die(err: Error) {
    panic!("{err}");
}
