use crate::Position;
use std::io::{stdout, Write, Error};
use crossterm::event::{read, Event, KeyEvent};

pub struct Size
{
    pub width: u16,
    pub height: u16,
}

pub struct Terminal
{
    pub size: Size,
}

impl Terminal {
    pub fn default() -> Result<Terminal, Error>
    {
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }
    pub fn read_key() -> Result<KeyEvent, Error>
    {
        loop {
            if let Event::Key(key) = read()? {
                return Ok(key);
            }
        }
    }

    pub fn cursor_position(position: &Position)
    {
        let Position{mut x, mut y} = position;
//        x = x.saturating_add(1);
//        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", crossterm::cursor::MoveTo(x, y));
    }

    pub fn clear_line()
    {
        print!("\x1b[2K");
    }

    pub fn flush() -> Result<(), Error>
    {
        stdout().flush()
    }
}
