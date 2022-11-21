use crate::Terminal;
use std::io::Error;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};

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
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            self.refresh_screen();
            if self.quit {
                println!("Hello");
                crossterm::terminal::disable_raw_mode().unwrap();
                let is_enable = match crossterm::terminal::is_raw_mode_enabled() {
                    Ok(enable) => enable,
                    Err(err) => {
                        die(err);
                        false
                    },
                };
                if is_enable == false {
                    break;
                }
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.quit = true,
            _ => (),
        }
        println!("Press");
        Ok(())
    }

    fn refresh_screen(&self) {
        Terminal::cursor_position(&self.cursor_position);
    }
}

fn die(err: Error) {
    panic!("{err}");
}
