use crate::terminal::Size;
use crate::Terminal;
use crate::Row;

pub struct Window
{
    size: Size,
}

impl Window
{
    pub fn default(terminal: &Terminal) -> Self
    {
        Self{
            size: Size {
                width: terminal.size.width,
                height: terminal.size.height,
            }
        }
    }
    pub fn render(&self)
    {
        for _ in 0..self.size.height
        {
            for x in 0..self.size.width
            {
                if x == 0 {
                    print!("~");
                } else {
                    print!(" ");
                }
            }
        }
    }

    pub fn line_render(&self, text: Option<&Row>)
    {
        if let Some(text) = text
        {
            print!("{}\r", text.string);
        }
        else
        {
            for x in 0..100
            {
                print!("A");
            }
        }
    }
}
