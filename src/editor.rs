use crate::Terminal;
use crate::Window;
use crate::Document;
use std::io::{stdout, Write, Error};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor
{
    quit: bool,
    cursor_position: Position,
    document: Document,
    terminal: Terminal,
    offset: Position,
}

impl Default for Editor
{
    fn default() -> Self
    {
        Self {
            quit: false,
            cursor_position: Position::default(),
            document: Document::default(),
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            offset: Position::default(),
        }
    }
}

impl Editor {
    pub fn run(&mut self)
    {
        crossterm::terminal::enable_raw_mode().unwrap();
        let window = Window::default(&self.terminal);
        window.render();
        loop {
            self.refresh_screen(&window);
            if self.quit {
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

    fn process_keypress(&mut self) -> Result<(), Error>
    {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.quit = true,
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.cursor_position.y = self.cursor_position.y.saturating_add(1),
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.cursor_position.y = self.cursor_position.y.saturating_sub(1),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.cursor_position.x = self.cursor_position.x.saturating_sub(1),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.cursor_position.x = self.cursor_position.x.saturating_add(1),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self, window: &Window) -> Result<(), Error>
    {
        //println!("{}, {}", &self.cursor_position.x, &self.cursor_position.y);
        Terminal::cursor_position(&Position {x: 0, y: self.cursor_position.y});
        //Terminal::clear_line();
        window.line_render(self.document.row(self.cursor_position.y));
        Terminal::cursor_position(&self.cursor_position);
        
        /*
        for _ in 0..self.terminal.size.height{
            println!("~\r");
        }
        */
        
        Terminal::flush()
    }
/*
    fn draw_row(&self, row: &Row)
    {
        let width = self.terminal.size().width;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.line_render(start, end);
    }

    fn draw_rows(&self)
    {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1
        {
            if let Some(row) = self.document.row(terminal_row)
            {
                self.draw_row(row);
            }
        }
    }
    */
}

fn die(err: Error)
{
    panic!("{err}");
}
