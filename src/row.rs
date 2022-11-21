pub struct Row
{
    string: String
}

impl Row
{
    pub fn default() -> Self
    {
        Self {
            string: String::new()
        }
    }
}
