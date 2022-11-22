pub struct Row
{
    pub string: String
}

impl Row
{
    pub fn default() -> Self
    {
        Self {
            string: String::new()
        }
    }
    pub fn new(text: &str) -> Self
    {
        Self {
            string: String::from(text)
        }
    }
}
