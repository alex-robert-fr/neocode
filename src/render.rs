use crate::Terminal;

pub struct Window
{
    screen: Vec<Vec<char>>,
}

impl Window
{
    pub fn default(terminal: &Terminal) -> Self
    {

        let mut screen = Vec::new();
        for _ in 0..terminal.size.height
        {
            let mut line = Vec::new();
            for _ in 0..terminal.size.width
            {
                line.push(' ');
            }
            screen.push(line);
        }
        Self{
            screen
        }
    }
    pub fn render(&self)
    {
        for y in self.screen.iter()
        {
            for (index, x) in y.iter().enumerate()
            {
                if index == 0 {
                    print!("~");
                } else {
                    print!("{x}");
                }
            }
        }
    }

    pub fn line_render(&self)
    {
        for x in 0..100
        {
            print!("A");
        }
    }
}
