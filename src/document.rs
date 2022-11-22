use crate::Row;
use crate::Terminal;

pub struct Document
{
    rows: Vec<Row>,
}

impl Document
{
    pub fn default() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::new("Hello world!"));
        rows.push(Row::new("Salut"));
        rows.push(Row::new("WHAT THE FUCK"));
        rows.push(Row::new("YYYASSS!"));
        rows.push(Row::new(""));
        rows.push(Row::new("Test ?!"));
        Self {
            rows
        }
    } 

    pub fn row(&self, index: usize) -> Option<&Row>
    {
        self.rows.get(index)
    }
}
